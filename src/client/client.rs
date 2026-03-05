use crate::client::ClientConfig;
use crate::common::Command;
use anyhow::Result;
use tokio::io::{AsyncReadExt, self, AsyncBufReadExt, BufReader};
use tokio::net::TcpStream;
use std::io::Write;

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
        let stdin = io::stdin();
        let mut reader = BufReader::new(stdin);

        loop {
            input.clear();
            print!("> ");
            std::io::stdout().flush().expect("couldn't flush");

            reader.read_line(&mut input).await?;
            let input = input.trim();
            if input.eq_ignore_ascii_case("exit") {
                break;
            }
            let command: Command = Command::parse(&input)?;
            println!("Parsed command: {:?}", command);
            Command::send(stream, command).await?;

            println!("Command sent, waiting for response...");

            // todo: proper framing while receiving
            let mut buffer = [0u8; 1024];
            let n = stream.read(&mut buffer).await?;
            let response = String::from_utf8_lossy(&buffer[..n]);
            println!("Response: {}", response);
        }

        Ok(())
    }
}
