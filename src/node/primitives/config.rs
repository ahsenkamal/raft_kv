use std::net::SocketAddr;

#[derive(Debug, Clone, Copy)]
pub struct NodeConfig {
    pub node_addr: SocketAddr,
    pub multicast_addr: SocketAddr,
}
