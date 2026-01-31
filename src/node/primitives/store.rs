use crate::common::Command;
use anyhow::Result;
use std::collections::HashMap;

pub struct KeyValueStore {
    store: HashMap<String, String>,
}

impl KeyValueStore {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn execute(&mut self, command: Command) -> Result<()> {
        match command {
            Command::GET { key } => {}
            Command::DEL { key } => {}
            Command::SET { key, value } => {}
        }
        Ok(())
    }
}
