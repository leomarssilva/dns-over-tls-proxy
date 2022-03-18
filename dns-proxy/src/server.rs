use crate::dns::message::Message;
use crate::dns::record::ResourceRecord;
use crate::dns::MessageBytes;
use crate::{Header, Question, ResponseCode};
use bytes::{Bytes, BytesMut};
use log::{debug, info};
use std::io;
use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::RwLock;
use tokio_native_tls::native_tls::{Certificate, TlsConnector};
use ttl_cache::TtlCache;

use std::env;
use std::rc::Rc;

#[derive(Clone)]
pub struct Cache {
    url: String,
    tlscontext: Arc<tokio_native_tls::TlsConnector>,
    answers: Arc<RwLock<TtlCache<Question, Vec<ResourceRecord>>>>,
}

impl Cache {
    pub fn new(size: usize, url: String, certificate: Bytes) -> Self {
        Cache {
            url,
            tlscontext: Arc::new(tokio_native_tls::TlsConnector::from(
                TlsConnector::builder()
                    .add_root_certificate(Certificate::from_pem(&certificate).unwrap())
                    .build()
                    .expect("Error building certificate"),
            )),
            answers: Arc::new(RwLock::new(TtlCache::new(size))),
        }
    }

    pub async fn get_entry(&self, question: Question) -> Vec<ResourceRecord> {
        let lock = self.answers.read().await;
        let value = lock.get(&question);

        match value {
            Some(x) => x.to_vec(),
            None => {
                let tlsconn = self.tlscontext.clone();
                let new_value = get_from_tls(self.url.clone(), tlsconn, question.clone())
                    .await
                    .expect("Error downloading data");
                if new_value.len() > 0 {
                    let ttl = (&new_value)[0].ttl as u64;
                    drop(lock);
                    let mut lock = self.answers.write().await;
                    lock.insert(question, new_value.clone(), Duration::from_secs(ttl));
                }
                new_value
            }
        }
    }
}

pub async fn process_bytes(buffer: Bytes, cache: Cache) -> Bytes {
    debug!("{:?}", &buffer);
    let mut packet = MessageBytes::from_bytes(buffer);
    let m = Message::parse(&mut packet);
    debug!("{:?}", m);

    let mut answer = vec![];
    for q in m.question.clone() {
        debug!("Getting answer for {:?}", q);
        let mut vrr = cache.get_entry(q).await;
        answer.append(&mut vrr);
    }

    Message {
        header: Header {
            question_response: 1,
            answer_count: answer.len() as u16,
            ..m.header
        },
        answer,
        ..m
    }
    .write(BytesMut::new())
    .freeze()
}

pub async fn process_tcp(
    mut socket: TcpStream,
    client_address: SocketAddr,
    cache: Cache,
) -> io::Result<()> {
    info!("Client {} connected", client_address);

    let size = socket.read_u16().await? as usize;
    let mut buffer = BytesMut::with_capacity(size);
    socket.read_buf(&mut buffer).await?;

    let mut new_message = process_bytes(buffer.freeze(), cache.clone()).await;

    socket.write_u16(new_message.len() as u16).await?;
    socket.write_buf(&mut new_message).await?;

    Ok(())
}

async fn get_from_tls(
    address: String,
    tlsconn: Arc<tokio_native_tls::TlsConnector>,
    question: Question,
) -> std::io::Result<Vec<ResourceRecord>> {
    let socket = TcpStream::connect(address.clone()).await?;

    let mut socket = tlsconn
        .connect(&address, socket)
        .await
        .expect("Error connecting over TLS");

    let msg = Message {
        header: Header {
            id: 123,
            question_response: 0,
            opcode: 0,
            authoritative_answer: 0,
            truncation: false,
            recursion_desired: true,
            recursion_available: false,
            z: 0,
            response_code: ResponseCode::NoError,
            question_count: 1,
            answer_count: 0,
            nameserver_count: 0,
            additional_records_count: 0,
        },
        question: vec![question],
        answer: vec![],
        authority: vec![],
        additional_records: vec![],
    }
    .write(BytesMut::new())
    .freeze();

    socket.write_u16(msg.len() as u16).await?;
    socket.write_all(msg.as_ref()).await?;

    let response_size = socket.read_u16().await? as usize;
    let mut data = BytesMut::with_capacity(response_size);
    socket.read_buf(&mut data).await?;

    // println!("{:?}", data);
    let mut pm = MessageBytes::from_bytes(data.freeze());
    let result = Message::parse(&mut pm);
    // println!("{:#?}", result);

    Ok(result.answer)
}
