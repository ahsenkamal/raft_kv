use std::{env, net::SocketAddr};
use anyhow::{Result, anyhow};
use raft_kv::client::{Client, ClientConfig};

fn parse_args() -> Result<ClientConfig> {
    let mut args = env::args().skip(1);

    let gateway_addr: SocketAddr = match args.next() {
        Some(s) => {
            s.parse()?
        }
        None => {
            return Err(anyhow!("please provide gateway address"));
        }
    };

    Ok(ClientConfig {
        gateway_addr,
    })
}

fn main() -> Result<()>{
    println!("client");
    let config = parse_args()?;
    let client = Client::new(config)?;
    client.start()?;
    Ok(())
}