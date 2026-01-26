use anyhow::{Result, anyhow};
use raft_kv::node::{Node, NodeConfig};
use std::env;

fn parse_args() -> Result<NodeConfig> {
    let mut args = env::args().skip(1);

    let node_port: u16 = match args.next() {
        Some(p) => p.parse()?,
        None => {
            return Err(anyhow!("please provide node port"));
        }
    };

    Ok(NodeConfig { node_port })
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("node");
    let config = parse_args()?;

    let mut node: Node = Node::new(config);
    node.start().await?;

    Ok(())
}
