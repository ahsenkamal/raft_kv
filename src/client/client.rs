use crate::client::ClientConfig;
use anyhow::Result;
use std::net::TcpStream;
use std::io;
use crate::common::Command;

pub struct Client {
    config: ClientConfig,
}

impl Client {
    pub fn new(config: ClientConfig) -> Result<Self> {
        // todo: validate config
        Ok(Self { config })
    }

    // starts the cli client
    pub fn start(&self) -> Result<()> {
        let mut stream = TcpStream::connect(self.config.gateway_addr)?;
        Self::repl(&mut stream)?;
        Ok(())
    }
    
    // infinite loop to parse and send commands to configured gateway
    fn repl(stream: &mut TcpStream) -> Result<(), anyhow::Error> {
        let mut input = String::new();
        loop {
            print!("> ");
            io::stdin().read_line(&mut input).expect("Faileld to take input");
            if input.eq_ignore_ascii_case("exit") {
                break;
            }
            let command: Command = Command::parse(&input)?;
            Command::send(stream, command)?;
        }

        Ok(())
    }
}