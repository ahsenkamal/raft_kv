use crate::node::primitives::NodeState;

pub fn candidate_start(state: &mut NodeState) {
    // vote for self
    // broadcast request for votes
    // wait for majority responses
    // if majority -> leader_start()
    // else step_down()
    state.make_candidate();
}
