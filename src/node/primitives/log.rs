use crate::common::Command;
use std::time::Instant;

pub struct LogEntry {
    timestamp: Instant,
    command: Command,
}