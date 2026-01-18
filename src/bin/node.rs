use std::{env, net::{TcpListener, UdpSocket}};
use anyhow::{Result, anyhow};
use raft_kv::node::{Node, NodeConfig};

fn parse_args() -> Result<NodeConfig> {
    let mut args = env::args().skip(1);

    let node_port: u16 = match args.next() {
        Some(p) => {
            p.parse()?
        }
        None => {
            return Err(anyhow!("please provide node port"));
        }
    };

    Ok(NodeConfig {
        node_port,
    })
}

fn main() -> Result<()> {
    println!("node");
    let config = parse_args()?;

    let addr = format!("0.0.0.0:{}", config.node_port);
    let discovery_socket = UdpSocket::bind(&addr)?;
    let node_socket = TcpListener::bind(&addr)?;

    let node: Node = Node::new(config)?;
    node.start()?;
    
    Ok(())
}