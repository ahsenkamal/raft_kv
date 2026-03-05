use anyhow::{Result, anyhow};
use raft_kv::node::{Node, NodeConfig};
use std::{env, net::SocketAddr};

fn parse_args() -> Result<NodeConfig> {
    let mut args = env::args().skip(1);

    let node_name: String = match args.next() {
        Some(name) => name,
        None => {
            return Err(anyhow!("please provide node name"));
        }
    };

    let node_addr: SocketAddr = match args.next() {
        Some(p) => SocketAddr::from(([0, 0, 0, 0], p.parse()?)),
        None => {
            return Err(anyhow!("please provide node port"));
        }
    };

    let multicast_addr: SocketAddr = match args.next() {
        Some(addr) => addr.parse()?,
        None => SocketAddr::from(([239, 255, 0, 1], 8999)),
    };

    Ok(NodeConfig {
        node_name,
        node_addr,
        multicast_addr,
    })
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = parse_args()?;
    println!("node bound at port: {}", config.node_addr);
    
    let mut node: Node = Node::new(config);
    node.start().await?;

    Ok(())
}
