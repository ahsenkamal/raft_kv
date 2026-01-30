use crate::net::discovery::handle_discovery;
use crate::net::messaging::handle_messaging;
use crate::node::modes::NodeMode;
use crate::node::modes::candidate;
use crate::node::modes::follower;
use crate::node::primitives::{
    KeyValueStore, LogEntry, NodeConfig, NodeEvent, NodeSnapshot, NodeState,
};
use anyhow::Result;
use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::sync::mpsc;

pub struct Node {
    config: NodeConfig,
    state: NodeState,
    committed_log: Vec<LogEntry>,
    uncommitted_log: Vec<LogEntry>,
    store: KeyValueStore,
    snapshot: Option<NodeSnapshot>,
    nodes: HashMap<String, SocketAddr>,
    connections: HashMap<SocketAddr, TcpStream>,
}

impl Node {
    pub fn new(config: NodeConfig) -> Self {
        // todo: validate config
        let state = NodeState::default();
        let committed_log = Vec::new();
        let uncommitted_log = Vec::new();
        let store = KeyValueStore::new();
        let snapshot = None;
        let nodes = HashMap::new();
        let connections = HashMap::new();

        Self {
            config,
            state,
            committed_log,
            uncommitted_log,
            store,
            snapshot,
            nodes,
            connections,
        }
    }

    async fn add_new_node(&mut self, node_name: String, addr: SocketAddr) -> Result<()> {
        self.nodes.insert(node_name, addr);
        let stream = TcpStream::connect(addr).await?;
        self.connections.insert(addr, stream);
        Ok(())
    }

    pub async fn start(&mut self) -> Result<()> {
        let (tx, mut rx) = mpsc::channel::<NodeEvent>(10);

        tokio::spawn(handle_discovery(self.config, tx.clone()));
        // tokio::spawn(handle_messaging(&self.addr));

        // Event loop
        loop {
            tokio::select! {
                Some(event) = rx.recv() => {
                    match event {
                        NodeEvent::NewNode(node_name, addr) => {
                            let _ = self.add_new_node(node_name, addr);
                        }
                        NodeEvent::LogEntry => {
                            self.state.reset_timeout_timer();
                        }
                        NodeEvent::VoteReqReceived(addr, new_term) => {
                            if self.state.get_mode() != NodeMode::Follower
                            || new_term <= self.state.get_term() {
                                continue;
                            }

                            if let Some(stream) = self.connections.get_mut(&addr) {
                                if follower::send_vote(stream).await.is_ok() {
                                    self.state.update_voted_term(new_term);
                                }
                            }

                            self.state.reset_timeout_timer();
                        }
                        NodeEvent::VoteReceived => {
                            if self.state.get_mode() != NodeMode::Candidate {
                                continue;
                            }

                            self.state.add_vote();
                            let majority_nodes = (self.connections.len() + 1)/2;

                            if self.state.get_votes() > majority_nodes as u32 {
                                self.state.init_leader();
                            }
                        }
                    }
                }
                _ = self.state.timeout_check() => {
                    match self.state.get_mode() {
                        NodeMode::Follower => {
                            self.state.init_candidate();
                            candidate::request_votes(&mut self.connections, self.state.get_term()).await;
                        }
                        NodeMode::Candidate => {
                            let majority_nodes = (self.connections.len() + 1)/2;

                            if self.state.get_votes() < majority_nodes as u32 {
                                self.state.init_follower();
                            }
                        }
                        NodeMode::Leader => {
                            self.state.reset_timeout_timer();
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
