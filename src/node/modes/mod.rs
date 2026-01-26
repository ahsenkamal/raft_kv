mod candidate;
mod follower;
mod leader;

pub enum NodeMode {
    Follower,
    Candidate,
    Leader,
}

pub use candidate::candidate_start;
pub use follower::follower_start;
