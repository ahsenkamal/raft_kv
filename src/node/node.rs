use crate::net::discovery::handle_discovery;
use crate::net::messaging::handle_messaging;
use crate::node::modes::{NodeMode, candidate_start, follower_start};
use crate::node::primitives::{
    KeyValueStore, LogEntry, NodeConfig, NodeEvent, NodeSnapshot, NodeState,
};
use anyhow::Result;
use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::sync::mpsc;

pub struct Node {
    config: NodeConfig,
    state: NodeState,
    committed_log: Vec<LogEntry>,
    uncommitted_log: Vec<LogEntry>,
    store: KeyValueStore,
    snapshot: Option<NodeSnapshot>,
    nodes: HashMap<String, SocketAddr>,
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

        Self {
            config,
            state,
            committed_log,
            uncommitted_log,
            store,
            snapshot,
            nodes,
        }
    }

    fn add_new_node(&mut self, node_name: String, addr: SocketAddr) {
        self.nodes.insert(node_name, addr);
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
                            self.add_new_node(node_name, addr);
                        }
                        NodeEvent::LogEntry => {

                        }
                        NodeEvent::VoteReq => {

                        }
                    }
                }
                _ = self.state.timeout_check() => {
                    match self.state.mode {
                        NodeMode::Follower => {
                            self.state.init_candidate();
                            candidate_start(&self.nodes);
                        }
                        NodeMode::Candidate => {
                            follower_start();
                        }
                        NodeMode::Leader => {}
                    }
                }
            }
        }

        Ok(())
    }
}
