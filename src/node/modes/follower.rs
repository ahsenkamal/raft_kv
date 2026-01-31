use anyhow::Result;
use tokio::net::TcpStream;

use crate::net::Packet;

pub async fn send_vote(stream: &mut TcpStream, term: u32) -> Result<()> {
    let payload = Vec::from(term.to_be_bytes());
    let packet = Packet::from_bytes(crate::net::PacketType::Vote, payload);
    Packet::send(stream, packet).await
}

pub async fn process_entries() {}
