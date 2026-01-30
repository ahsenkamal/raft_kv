use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub async fn heartbeat(connections: &mut HashMap<SocketAddr, TcpStream>) {
    for (_, stream) in connections.iter_mut() {
        let hb = b"hb";
        let _ = stream.write_all(hb).await;
    }
}
