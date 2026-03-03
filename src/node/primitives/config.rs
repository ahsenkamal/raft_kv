use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct NodeConfig {
    pub node_name: String,
    pub node_addr: SocketAddr,
    pub multicast_addr: SocketAddr,
}
