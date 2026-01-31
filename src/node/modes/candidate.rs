use std::{collections::HashMap, net::SocketAddr};
use tokio::net::TcpStream;

use crate::net::Packet;

pub async fn request_votes(connections: &mut HashMap<SocketAddr, TcpStream>, term: u32) {
    let payload = Vec::from(term.to_be_bytes());
    let packet = Packet::from_bytes(crate::net::PacketType::VoteReq, payload);

    for (_, stream) in connections.iter_mut() {
        let _ = Packet::send(stream, packet.clone()).await;
    }
}
