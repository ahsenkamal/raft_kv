use std::collections::HashMap;
use std::net::SocketAddr;

pub fn candidate_start(nodes: &HashMap<String, SocketAddr>) {
    // [ ] broadcast request for votes
    // [ ] wait for majority responses
    // [ ] if majority -> leader_start()
    // [ ] else step_down()
}
