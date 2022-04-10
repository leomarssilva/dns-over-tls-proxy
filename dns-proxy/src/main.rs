extern crate core;

mod dns;
mod logger;
mod server;

use std::env;

use bytes::Bytes;
use log::{debug, info};
use std::io;
use std::net::Ipv4Addr;
use std::str::FromStr;
use tokio::fs;
use tokio::net::{TcpListener, UdpSocket};

use crate::dns::header::*;
use crate::dns::question::Question;
use crate::server::Cache;

async fn loop_tcp(address: (Ipv4Addr, u16), cache: Cache) -> io::Result<()> {
    let tcp_listener = TcpListener::bind(address).await?;

    info!("TCP server listening on {}:{}", address.0, address.1);

    loop {
        let (socket, client_address) = tcp_listener.accept().await?;
        let c_cache = cache.clone();
        tokio::spawn(async move {
            server::process_tcp(socket, client_address, c_cache)
                .await
                .expect("Error");
        });
    }
}

async fn loop_udp(address: (Ipv4Addr, u16), cache: Cache) -> io::Result<()> {
    let socket = UdpSocket::bind(&address).await?;

    info!("UDP server listening on {}:{}", address.0, address.1);

    loop {
        // let mut buffer = BytesMut::with_capacity(512);
        let mut buffer = vec![0u8; 512];
        let (size, client_address) = socket.recv_from(&mut buffer).await?;

        debug!("udp pack size {}", size);
        if size == 0 {
            continue;
        }

        // due to the way udp sockets work, concurrent requests need to be implemented in a different way (e.g. FuturesUnordered)
        let result = server::process_bytes(Bytes::from(buffer), cache.clone()).await;

        socket.send_to(&result, client_address).await?;
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    logger::setup_logger().expect("Error setting log");

    let upstream_name =
        env::var("DOT_SERVER_NAME").expect("Need to set DOT_SERVER_NAME");
    let upstream_address =
        env::var("DOT_SERVER_ADDRESS").expect("Need to set DOT_SERVER_ADDRESS");
    // let certificate = env::var("CERTIFICATE").expect("Need to set CERTIFICATE (PEM)");
    // let certificate_contents = fs::read_to_string(certificate).await?;
    let cache = Cache::new(100, upstream_address, upstream_name);

    let ip = "0.0.0.0";
    let port = u16::from_str(&env::var("PORT").unwrap_or(String::from("53"))).unwrap();
    let address: (Ipv4Addr, u16) = (ip.parse().unwrap(), port);

    let c_cache1 = cache.clone();
    let c_cache2 = cache.clone();
    tokio::join!(
        async move {
            loop_tcp(address, c_cache1).await.expect("Error TCP");
        },
        async move {
            loop_udp(address, c_cache2).await.expect("Error UDP");
        }
    );

    Ok(())
}
