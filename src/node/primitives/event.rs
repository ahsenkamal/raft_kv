use std::net::SocketAddr;

pub enum NodeEvent {
    NewNode(String, SocketAddr),
    LogEntry,
    VoteReqReceived,
    VoteReceived,
}
