use crate::net::discovery::handle_discovery;
use crate::net::messaging::handle_messaging;
use crate::node::primitives::{KeyValueStore, LogEntry, NodeConfig, NodeSnapshot, NodeState};
use anyhow::Result;

pub struct Node {
    config: NodeConfig,
    addr: String,
    state: NodeState,
    committed_log: Vec<LogEntry>,
    uncommitted_log: Vec<LogEntry>,
    store: KeyValueStore,
    snapshot: Option<NodeSnapshot>,
}

impl Node {
    pub fn new(config: NodeConfig) -> Self {
        // todo: validate config
        let addr = format!("0.0.0.0:{}", config.node_port);
        let state: NodeState = NodeState::default();
        let committed_log: Vec<LogEntry> = Vec::new();
        let uncommitted_log: Vec<LogEntry> = Vec::new();
        let store: KeyValueStore = KeyValueStore::new();
        let snapshot = None;

        Self {
            config,
            addr,
            state,
            committed_log,
            uncommitted_log,
            store,
            snapshot,
        }
    }

    async fn handle_election() {}

    pub async fn start(&self) -> Result<()> {
        tokio::spawn(handle_discovery(&self.addr));
        tokio::spawn(handle_messaging(&self.addr));
        tokio::spawn(Self::handle_election());

        Ok(())
    }
}
