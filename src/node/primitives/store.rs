use crate::common::Command;
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug)]
pub struct KeyValueStore {
    store: HashMap<String, String>,
}

impl KeyValueStore {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn execute(&mut self, command: Command) -> Option<String> {
        match command {
            Command::GET { key } => {
                self.store.get(&key).cloned()
            }
            Command::DEL { key } => {
                self.store.remove(&key);
                None
            }
            Command::SET { key, value } => {
                self.store.insert(key, value);
                None
            }
        }
    }
}
