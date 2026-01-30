use std::net::SocketAddr;

pub enum NodeEvent {
    NewNode(String, SocketAddr),
    LogEntry,
    VoteReqReceived(SocketAddr, u32),
    VoteReceived,
}
