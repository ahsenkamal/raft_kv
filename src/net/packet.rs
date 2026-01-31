use crate::net::PacketType;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Packet {
    pub packet_type: PacketType,
    pub payload: Vec<u8>,
}

impl Packet {
    pub async fn from_stream(stream: &mut TcpStream) -> Result<Packet> {
        // read packet from stream

        let mut len_buf = [0u8; 4];
        stream.read_exact(&mut len_buf).await?;
        let len = u32::from_be_bytes(len_buf) as usize;

        let mut packet_buf = vec![0u8; len];
        stream.read_exact(&mut packet_buf).await?;

        let packet: Packet = bincode::deserialize(&packet_buf)?;

        return Ok(packet);
    }

    pub fn from_bytes(packet_type: PacketType, payload: Vec<u8>) -> Packet {
        Packet {
            packet_type,
            payload,
        }
    }

    pub async fn send(stream: &mut TcpStream, packet: Packet) -> Result<()> {
        let packet_bytes = bincode::serialize(&packet)?;
        let len = (packet_bytes.len() as u32).to_be_bytes();

        stream.write_all(&len).await?;
        // drop connection on any error
        // also: can write both of these in one call
        stream.write_all(&packet_bytes).await?;

        Ok(())
    }
}
