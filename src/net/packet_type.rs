use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum PacketType {
    Discovery,
    ClientReq,
    ClientRes,
    LogEntry,
}
