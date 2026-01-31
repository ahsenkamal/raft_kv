use anyhow::Result;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::sync::mpsc;

use crate::net::Packet;
use crate::net::PacketType;
use crate::node::NodeConfig;
use crate::node::NodeEvent;

pub async fn handle_messaging(config: NodeConfig, tx: mpsc::Sender<NodeEvent>) -> Result<()> {
    let node_socket = TcpListener::bind(config.node_addr).await?;

    loop {
        let (stream, _) = node_socket.accept().await?;
        let tx_clone = tx.clone();
        tokio::spawn(async move {
            let _ = handle_connection(stream, tx_clone).await;
        });
    }
}

async fn handle_connection(mut stream: TcpStream, tx: mpsc::Sender<NodeEvent>) -> Result<()> {
    loop {
        if let Ok(packet) = Packet::from_stream(&mut stream).await {
            handle_packet(packet, tx.clone()).await;
        }
    }
}

async fn handle_packet(packet: Packet, tx: mpsc::Sender<NodeEvent>) {
    match packet.packet_type {
        PacketType::ClientReq => {}
        PacketType::ClientRes => {}
        PacketType::Discovery => {}
        PacketType::LogEntry => {}
        PacketType::VoteReq => {}
        PacketType::Vote => {}
    }
}
