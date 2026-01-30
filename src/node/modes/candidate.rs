use std::{collections::HashMap, net::SocketAddr};
use tokio::{io::AsyncWriteExt, net::TcpStream};

pub async fn request_votes(connections: &mut HashMap<SocketAddr, TcpStream>) {
    for (_, stream) in connections.iter_mut() {
        let vote_req = b"vote";
        let _ = stream.write_all(vote_req).await;
    }
}
