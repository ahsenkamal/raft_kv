use crate::node::primitives::{KeyValueStore, LogEntry, NodeConfig, NodeSnapshot, NodeState};
use anyhow::Result;

pub struct Node {
    config: NodeConfig,
    state: NodeState,
    committed_log: Vec<LogEntry>,
    uncommitted_log: Vec<LogEntry>,
    store: KeyValueStore,
    snapshot: Option<NodeSnapshot>,
}

impl Node {
    pub fn new(config: NodeConfig) -> Self {
        // todo: validate config
        let state: NodeState = NodeState::default();
        let committed_log: Vec<LogEntry> = Vec::new();
        let uncommitted_log: Vec<LogEntry> = Vec::new();
        let store: KeyValueStore = KeyValueStore::new();
        let snapshot = None;

        Self {
            config,
            state,
            committed_log,
            uncommitted_log,
            store,
            snapshot,
        }
    }

    pub fn start(&self) -> Result<()> {
        Ok(())
    }
}
