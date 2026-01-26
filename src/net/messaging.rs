use tokio::net::TcpListener;

pub async fn handle_messaging(addr: &str) {
    let node_socket = TcpListener::bind(addr).await;
}
