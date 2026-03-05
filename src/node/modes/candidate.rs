use std::{collections::HashMap, net::IpAddr};
use tokio::net::TcpStream;

use crate::net::Packet;

pub async fn request_votes(connections: &mut HashMap<IpAddr, TcpStream>, term: u32) {
    println!("Sending vote requests to all nodes");
    let payload = Vec::from(term.to_be_bytes());
    let packet = Packet::from_bytes(crate::net::PacketType::VoteReq, payload);

    for (_, stream) in connections.iter_mut() {
        println!("Sending vote request to {}", stream.peer_addr().unwrap());
        let _ = Packet::send(stream, packet.clone()).await;
    }
}
