use std::net::SocketAddr;

use crate::{common::Command, node::primitives::LogEntry};

pub enum NodeEvent {
    NewNode(String, SocketAddr),
    LogEntry(u32, Vec<LogEntry>),
    LogAck(u32, String),
    VoteReqReceived(SocketAddr, u32),
    VoteReceived,
    ClientReq(Command),
}
