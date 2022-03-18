extern crate core;

mod dns;
mod logger;
mod tcp_server;

use log::info;
use std::io;
use std::net::Ipv4Addr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    logger::setup_logger().expect("Error setting log");

    let ip = "0.0.0.0";
    let port = 5553;
    let address: (Ipv4Addr, u16) = (ip.parse().unwrap(), port);
    let tcp_listener = TcpListener::bind(address)
        .await
        .expect("Unable to bind tcp server");
    info!("Server listening on {}:{}", address.0, address.1);
    loop {
        let (socket, client_address) = tcp_listener.accept().await?;
        tokio::spawn(async move {
            tcp_server::process(socket, client_address).await;
        });
    }
}
