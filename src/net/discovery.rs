use crate::node::{NodeConfig, NodeEvent};
use anyhow::Result;
use anyhow::anyhow;
use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::mpsc;
use tokio::time::{Duration, interval};

async fn discovery_beacon(node_name: String, node_addr: SocketAddr, socket: Arc<UdpSocket>, multicast_addr: SocketAddr) {
    let mut ticker = interval(Duration::from_secs(1));
    loop {
        ticker.tick().await;
        // todo: send node_addr too
        let port = node_addr.port().to_be_bytes();
        let msg = node_name.as_bytes();
        let mut payload = Vec::new();
        payload.extend_from_slice(&port);
        payload.extend_from_slice(msg);

        // todo: send a proper packet
        let _ = socket.send_to(&payload, multicast_addr).await;
        println!("sending {}", node_name);
    }
}

pub async fn handle_discovery(config: NodeConfig, tx: mpsc::Sender<NodeEvent>) -> Result<()> {
    let discovery_socket = UdpSocket::bind("0.0.0.0:8999").await?;
    println!("bound udp 8999");
    let multi_ip = match config.multicast_addr.ip() {
        std::net::IpAddr::V4(addr) => addr,
        std::net::IpAddr::V6(_) => return Err(anyhow!("error parsing multicast ip")),
    };
    if let Err(e) = discovery_socket.join_multicast_v4(multi_ip, Ipv4Addr::UNSPECIFIED) {
        return Err(anyhow!("couldn't join multicast {}", e));
    }
    let shared_discovery_socket = Arc::new(discovery_socket);
    let node_name = config.node_name.clone();
    let node_addr = config.node_addr.clone();

    tokio::spawn(discovery_beacon(
        node_name,
        node_addr,
        shared_discovery_socket.clone(),
        config.multicast_addr,
    ));

    let mut buf = [0u8; 1024];
    loop {
        println!("waiting for packets...");
        match shared_discovery_socket.recv_from(&mut buf).await {
            Ok((len, src)) => {
                // todo: receive a proper packet
                let port = u16::from_be_bytes(buf[..2].try_into().unwrap());
                let msg = String::from_utf8_lossy(&buf[2..]).into_owned();
                let new_addr = SocketAddr::new(src.ip(), port);
                println!("found {}", new_addr);

                let _ = tx.send(NodeEvent::NewNode(msg, new_addr)).await;
            }
            Err(e) => {
                println!("udp recv error: {}", e);
            }
        }
    }
}
