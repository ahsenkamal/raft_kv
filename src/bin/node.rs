use anyhow::{Result, anyhow};
use raft_kv::node::{Node, NodeConfig};
use std::{env, net::SocketAddr};

fn parse_args() -> Result<NodeConfig> {
    let mut args = env::args().skip(1);

    let node_addr: SocketAddr = match args.next() {
        Some(p) => SocketAddr::from(([0, 0, 0, 0], p.parse()?)),
        None => {
            return Err(anyhow!("please provide node port"));
        }
    };

    let multicast_addr: SocketAddr = match args.next() {
        Some(addr) => addr.parse()?,
        None => SocketAddr::from(([239, 255, 0, 1], 3000)),
    };

    Ok(NodeConfig {
        node_addr,
        multicast_addr,
    })
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("node");
    let config = parse_args()?;

    let mut node: Node = Node::new(config);
    node.start().await?;

    Ok(())
}
