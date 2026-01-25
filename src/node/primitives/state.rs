use std::time::{Duration, Instant};

pub enum NodeMode {
    Follower,
    Candidate,
    Leader,
}

pub struct NodeState {
    mode: NodeMode,
    timeout: Instant,
    term: u32,
}

impl NodeState {
    pub fn default() -> Self {
        Self {
            mode: NodeMode::Follower,
            timeout: Instant::now() + Duration::from_secs(3),
            term: 0,
        }
    }
}
