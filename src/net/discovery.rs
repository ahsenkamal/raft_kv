use crate::node::{NodeConfig, NodeEvent};
use anyhow::Result;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::mpsc;
use tokio::time::{Duration, interval};

async fn discovery_beacon(socket: Arc<UdpSocket>, multicast_addr: SocketAddr) {
    let mut ticker = interval(Duration::from_secs(1));
    loop {
        ticker.tick().await;
        let msg = b"raft_kv_node1";
        let _ = socket.send_to(msg, multicast_addr).await;
    }
}

pub async fn handle_discovery(config: NodeConfig, tx: mpsc::Sender<NodeEvent>) -> Result<()> {
    let discovery_socket = UdpSocket::bind(config.node_addr).await?;
    let shared_discovery_socket = Arc::new(discovery_socket);

    tokio::spawn(discovery_beacon(
        shared_discovery_socket.clone(),
        config.multicast_addr,
    ));

    let mut buf = [0u8; 1024];
    loop {
        match shared_discovery_socket.recv_from(&mut buf).await {
            Ok((len, src)) => {
                let _ = tx
                    .send(NodeEvent::NewNode(
                        String::from_utf8_lossy(&buf[..len]).into_owned(),
                        src,
                    ))
                    .await;
            }
            Err(e) => {
                println!("udp recv error: {}", e);
            }
        }
    }
}
