use std::net::TcpStream;
use crate::net::{Packet, PacketType};
use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    SET {key: String, value: String},
    GET {key: String},
    DEL {key: String},
}

impl Command {
    pub fn parse(input: &str) -> Result<Command> {
        let tokens: Vec<&str> = input.split_whitespace().collect();
        
        match tokens.as_slice() {
            [cmd, key, value] if cmd.eq_ignore_ascii_case("set") => {
                Ok(Command::SET {
                    key: key.to_string(),
                    value: value.to_string(),
                })
            }
    
            [cmd, key] if cmd.eq_ignore_ascii_case("get") => {
                Ok(Command::GET {
                    key: key.to_string(),
                })
            }
    
            [cmd, key] if cmd.eq_ignore_ascii_case("del") => {
                Ok(Command::DEL { key: key.to_string(), })
            }
    
            _ => {
                Err(anyhow!("Invalid command {}", input))
            }
        }
    }
    
    pub fn send(stream: &mut TcpStream, command: Command) -> Result<()>{
        let bytes = bincode::serialize(&command)?;
        let packet = Packet::from_bytes(PacketType::ClientReq, bytes);

        Packet::send(stream, packet)?;
        Ok(())
    }
}
