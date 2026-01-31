use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PacketType {
    Discovery,
    ClientReq,
    ClientRes,
    LogEntry,
    VoteReq,
    Vote,
}
