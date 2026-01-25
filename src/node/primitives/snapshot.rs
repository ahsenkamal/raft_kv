use crate::node::primitives::{KeyValueStore, LogEntry};

pub struct NodeSnapshot {
    last_entry: LogEntry,
    store_snap: KeyValueStore,
}

impl NodeSnapshot {}
