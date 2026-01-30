use std::{collections::HashMap, net::SocketAddr};
use tokio::{io::AsyncWriteExt, net::TcpStream};

pub async fn request_votes(connections: &mut HashMap<SocketAddr, TcpStream>, term: u32) {
    let mut vote_req = Vec::from(b"vote".as_slice());
    vote_req.extend_from_slice(&term.to_be_bytes());

    for (_, stream) in connections.iter_mut() {
        let _ = stream.write_all(&vote_req).await;
    }
}
