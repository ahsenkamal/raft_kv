use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::net::TcpStream;

use crate::net::Packet;
use crate::node::primitives::LogEntry;

pub async fn heartbeat(
    connections: &mut HashMap<SocketAddr, TcpStream>,
    term: u32,
    entries: &Vec<LogEntry>,
) {
    let mut payload = Vec::from(term.to_be_bytes());
    let mut entries_bytes = bincode::serialize(entries).unwrap_or(vec![0u8]);
    payload.append(&mut entries_bytes);

    let packet = Packet::from_bytes(crate::net::PacketType::LogEntry, payload);
    for (_, stream) in connections.iter_mut() {
        let _ = Packet::send(stream, packet.clone()).await;
    }
}
