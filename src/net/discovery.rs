use anyhow::Result;
use tokio::net::UdpSocket;
use tokio::time::{Duration, interval};

async fn discovery_beacon(socket: UdpSocket) {
    let mut ticker = interval(Duration::from_secs(1));
    loop {
        ticker.tick().await;
    }
}

pub async fn handle_discovery(addr: &str) -> Result<()> {
    let discovery_socket = UdpSocket::bind(addr).await?;
    Ok(())
}
