use std::time::Duration;

use tokio::time::{Interval, interval};

pub enum NodeMode {
    Follower,
    Candidate,
    Leader,
}

pub struct NodeState {
    pub mode: NodeMode,
    pub timeout: Interval,
    pub term: u32,
}

impl NodeState {
    pub fn default() -> Self {
        Self {
            mode: NodeMode::Follower,
            timeout: interval(Duration::from_secs(3)),
            term: 0,
        }
    }
}
