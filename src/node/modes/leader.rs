use std::collections::HashMap;
use std::net::IpAddr;
use tokio::net::TcpStream;

use crate::net::Packet;
use crate::node::primitives::LogEntry;

pub async fn heartbeat(
    connections: &mut HashMap<IpAddr, TcpStream>,
    term: u32,
    entries: &Vec<LogEntry>,
) {
    println!("Sending heartbeat to all nodes");
    let mut payload = Vec::from(term.to_be_bytes());
    let mut entries_bytes = bincode::serialize(entries).unwrap_or(vec![0u8]);
    payload.append(&mut entries_bytes);

    let packet = Packet::from_bytes(crate::net::PacketType::LogEntry, payload);
    for (_, stream) in connections.iter_mut() {
        let _ = Packet::send(stream, packet.clone()).await;
    }
}

pub async fn send_log_committed(
    connections: &mut HashMap<IpAddr, TcpStream>,
) {
    println!("Sending log committed message to all nodes");
    let mut payload = Vec::new();

    let packet = Packet::from_bytes(crate::net::PacketType::LogCommitted, payload);
    for (_, stream) in connections.iter_mut() {
        let _ = Packet::send(stream, packet.clone()).await;
    }
}