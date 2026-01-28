use crate::node::modes::NodeMode;
use std::time::Duration;
use tokio::time::{Interval, interval};

pub struct NodeState {
    pub mode: NodeMode,
    pub timeout_timer: Interval,
    pub term: u32,
    pub votes: u32,
}

impl NodeState {
    pub fn default() -> Self {
        Self {
            mode: NodeMode::Follower,
            timeout_timer: interval(Duration::from_secs(3)),
            term: 0,
            votes: 0,
        }
    }

    pub async fn timeout_check(&mut self) {
        self.timeout_timer.tick().await;
    }

    pub fn init_candidate(&mut self) {
        self.mode = NodeMode::Candidate;
        self.timeout_timer.reset();
        self.votes = 1;
    }

    pub fn init_leader(&mut self) {
        self.mode = NodeMode::Leader;
        self.timeout_timer.reset();
    }
}
