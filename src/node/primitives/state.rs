use crate::node::modes::NodeMode;
use std::time::Duration;
use tokio::time::{Interval, interval};

pub struct NodeState {
    mode: NodeMode,
    timeout_timer: Interval,
    term: u32,
    votes: u32,
    voted_term: u32,
}

impl NodeState {
    pub fn default() -> Self {
        Self {
            mode: NodeMode::Follower,
            timeout_timer: interval(Duration::from_secs(3)),
            term: 0,
            votes: 0,
            voted_term: 0,
        }
    }

    pub fn get_mode(&self) -> NodeMode {
        self.mode
    }

    pub fn get_votes(&self) -> u32 {
        self.votes
    }

    pub fn get_term(&self) -> u32 {
        self.term
    }

    pub fn get_voted_term(&self) -> u32 {
        self.voted_term
    }

    pub fn update_voted_term(&mut self, new_voted_term: u32) {
        self.voted_term = new_voted_term;
    }

    pub async fn timeout_check(&mut self) {
        self.timeout_timer.tick().await;
    }

    pub fn reset_timeout_timer(&mut self) {
        self.timeout_timer.reset();
    }

    pub fn add_vote(&mut self) {
        self.votes += 1;
    }

    pub fn init_candidate(&mut self) {
        self.timeout_timer = interval(Duration::from_secs(3));
        self.mode = NodeMode::Candidate;
        self.votes = 1;
        self.term += 1;
    }

    pub fn init_leader(&mut self) {
        self.timeout_timer = interval(Duration::from_secs(1));
        self.mode = NodeMode::Leader;
        self.votes = 0;
    }

    pub fn init_follower(&mut self, term: u32) {
        self.timeout_timer = interval(Duration::from_secs(3));
        self.mode = NodeMode::Follower;
        self.votes = 0;
        self.term = term;
    }
}
