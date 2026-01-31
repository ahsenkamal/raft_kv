use std::net::SocketAddr;

use crate::node::primitives::LogEntry;

pub enum NodeEvent {
    NewNode(String, SocketAddr),
    LogEntry(u32, Vec<LogEntry>),
    VoteReqReceived(SocketAddr, u32),
    VoteReceived,
}
