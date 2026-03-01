use anyhow::Result;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::io::AsyncWriteExt;
use tokio::sync::oneshot;

use crate::common::Command;
use crate::net::Packet;
use crate::net::PacketType;
use crate::node::NodeConfig;
use crate::node::NodeEvent;
use crate::node::LogEntry;

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
            handle_packet(&mut stream, packet, tx.clone()).await;
        }
    }
}

async fn handle_packet(stream: &mut TcpStream, packet: Packet, tx: mpsc::Sender<NodeEvent>) {
    match packet.packet_type {
        PacketType::ClientReq => {
            let command = Command::from_packet(packet);
            let (reply_tx, reply_rx) = oneshot::channel();
            let _ = tx.send(NodeEvent::ClientReq(reply_tx, command)).await;
            
            if let Ok(reply) = reply_rx.await {
                let _ = stream.write_all(reply.as_bytes()).await;
            }
        }
        PacketType::ClientRes => {
            // do nothing
        }
        PacketType::Discovery => {
            // do nothing
        }
        PacketType::LogEntry => {
            // todo: remove unwraps
            let term = u32::from_be_bytes(packet.payload[0..4].try_into().unwrap());
            let entries_bytes = &packet.payload[4..];
            let entries: Vec<LogEntry> = bincode::deserialize(entries_bytes).unwrap();
            let leader_addr = stream.peer_addr().unwrap();

            let _ = tx.send(NodeEvent::LogEntry(leader_addr, term, entries)).await;
        }
        PacketType::LogAck => {
            let _ = tx.send(NodeEvent::LogAck(0, String::new()));
        }
        PacketType::VoteReq => {
            let term_bytes: [u8; 4] = packet.payload[0..4].try_into().unwrap();
            let new_term = u32::from_be_bytes(term_bytes);
            let _ = tx.send(NodeEvent::VoteReqReceived(stream.peer_addr().unwrap(), new_term));
        }
        PacketType::Vote => {
            let _ = tx.send(NodeEvent::VoteReceived);
        }
    }
}
