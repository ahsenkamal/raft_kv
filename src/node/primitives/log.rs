use crate::common::Command;
use serde::{Deserialize, Serialize};
use tokio::time::Instant;

#[derive(Serialize, Deserialize, Debug)]
pub struct LogEntry {
    // timestamp: Instant,
    command: Command,
}

impl LogEntry {
    pub fn new(command: Command) -> Self {
        Self {
            // timestamp: Instant::now(),
            command,
        }
    }
}
