pub mod candidate;
pub mod follower;
pub mod leader;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum NodeMode {
    Follower,
    Candidate,
    Leader,
}
