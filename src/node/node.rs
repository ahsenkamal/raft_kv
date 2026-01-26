use crate::net::discovery::handle_discovery;
use crate::net::messaging::handle_messaging;
use crate::node::modes::{NodeMode, candidate_start, follower_start};
use crate::node::primitives::{KeyValueStore, LogEntry, NodeConfig, NodeSnapshot, NodeState};
use anyhow::Result;
use std::collections::HashMap;

pub struct Node {
    config: NodeConfig,
    addr: String,
    state: NodeState,
    committed_log: Vec<LogEntry>,
    uncommitted_log: Vec<LogEntry>,
    store: KeyValueStore,
    snapshot: Option<NodeSnapshot>,
    nodes: HashMap<String, u16>,
}

impl Node {
    pub fn new(config: NodeConfig) -> Self {
        // todo: validate config
        let addr = format!("0.0.0.0:{}", config.node_port);
        let state = NodeState::default();
        let committed_log = Vec::new();
        let uncommitted_log = Vec::new();
        let store = KeyValueStore::new();
        let snapshot = None;
        let nodes = HashMap::new();

        Self {
            config,
            addr,
            state,
            committed_log,
            uncommitted_log,
            store,
            snapshot,
            nodes,
        }
    }

    pub async fn start(&mut self) -> Result<()> {
        tokio::spawn(handle_discovery(&self.addr));
        tokio::spawn(handle_messaging(&self.addr));

        loop {
            self.state.timeout_check().await;

            match self.state.mode {
                NodeMode::Follower => {
                    candidate_start(&mut self.state);
                }
                NodeMode::Candidate => {
                    follower_start();
                }
                NodeMode::Leader => {}
            }
        }

        Ok(())
    }
}
