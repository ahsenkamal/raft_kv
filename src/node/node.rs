use crate::node::primitives::{
    NodeConfig,
    NodeState,
    LogEntry,
    KeyValueStore,
    NodeSnapshot,
};
use anyhow::Result;

pub struct Node {
    config: NodeConfig,
    state: NodeState,
    committed_log: Vec<LogEntry>,
    uncommitted_log: Vec<LogEntry>,
    store: KeyValueStore,
    snapshot: NodeSnapshot,
}

impl Node {
    pub fn new(config: NodeConfig) -> Result<Self> {
        // todo: validate config
        let state: NodeState = NodeState::default();
        let committed_log: Vec<LogEntry> = Vec::new();
        let uncommitted_log: Vec<LogEntry> = Vec::new();
        let store: KeyValueStore = KeyValueStore::new();

        Ok(Self { config, state })
    }
}