use crate::dns::message::Message;
use crate::dns::MessagePacket;
use bytes::{Bytes, BytesMut};
use log::info;
use std::io;
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub async fn process(mut socket: TcpStream, client_address: SocketAddr) -> io::Result<()> {
    info!("Client {} connected", client_address);
    let mut buff = [0; 1024];
    loop {
        let size = socket.read_u16().await? as usize;
        let mut buffer = BytesMut::with_capacity(size);
        socket.read_buf(&mut buffer).await?;

        println!("{:?}", &buffer);
        let mut packet = MessagePacket::from_bytes(buffer.freeze());
        let m = Message::parse(&mut packet);
        println!("{:?}", m);

        // open_connection(m)
    }
}
