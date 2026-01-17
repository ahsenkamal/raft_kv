use crate::net::PacketType;
use std::{io::Write, net::TcpStream};
use anyhow::Result;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Packet {
    packet_type: PacketType,
    payload: Vec<u8>,
}

impl Packet {
    pub fn from_stream(stream: &mut TcpStream) -> Result<Packet> {
        // read packet
        Ok(Packet { packet_type: PacketType::ClientReq, payload: b"a".to_vec() })
    }

    pub fn from_bytes(packet_type: PacketType, payload: Vec<u8>) -> Packet {
        Packet {
            packet_type,
            payload,
        }
    }

    pub fn send(stream: &mut TcpStream, packet: Packet) -> Result<()> {
        let packet_bytes = bincode::serialize(&packet)?;
        let len = (packet_bytes.len() as u32).to_be_bytes();

        stream.write_all(&len)?;
        // drop connection on any error
        // also: can write both of these in one call
        stream.write_all(&packet_bytes)?;

        Ok(())
    }
}
