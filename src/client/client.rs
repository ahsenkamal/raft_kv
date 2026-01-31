use crate::client::ClientConfig;
use crate::common::Command;
use anyhow::Result;
use std::io;
use tokio::net::TcpStream;

pub struct Client {
    config: ClientConfig,
}

impl Client {
    pub fn new(config: ClientConfig) -> Result<Self> {
        // todo: validate config
        Ok(Self { config })
    }

    // starts the cli client
    pub async fn start(&self) -> Result<()> {
        let mut stream = TcpStream::connect(self.config.gateway_addr).await?;
        Self::repl(&mut stream).await?;
        Ok(())
    }

    // infinite loop to parse and send commands to configured gateway
    async fn repl(stream: &mut TcpStream) -> Result<(), anyhow::Error> {
        let mut input = String::new();
        loop {
            print!("> ");
            io::stdin()
                .read_line(&mut input)
                .expect("Faileld to take input");
            if input.eq_ignore_ascii_case("exit") {
                break;
            }
            let command: Command = Command::parse(&input)?;
            Command::send(stream, command).await?;
        }

        Ok(())
    }
}
