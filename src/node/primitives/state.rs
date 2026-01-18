use std::time::Instant;

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