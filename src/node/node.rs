use crate::common::Command;
use crate::net::{Packet, PacketType};
use crate::net::discovery::handle_discovery;
use crate::net::messaging::{self, handle_messaging};
use crate::node::modes::NodeMode;
use crate::node::modes::{candidate, follower, leader};
use crate::node::primitives::{
    KeyValueStore, LogEntry, NodeConfig, NodeEvent, NodeSnapshot, NodeState,
};
use anyhow::Result;
use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::sync::mpsc;

pub struct Node {
    config: NodeConfig,
    state: NodeState,
    committed_log: Vec<LogEntry>,
    uncommitted_log: Vec<LogEntry>,
    store: KeyValueStore,
    snapshot: Option<NodeSnapshot>,
    nodes: HashMap<String, SocketAddr>,
    connections: HashMap<SocketAddr, TcpStream>,
}

impl Node {
    pub fn new(config: NodeConfig) -> Self {
        // todo: validate config
        let state = NodeState::default();
        let committed_log = Vec::new();
        let uncommitted_log = Vec::new();
        let store = KeyValueStore::new();
        let snapshot = None;
        let nodes = HashMap::new();
        let connections = HashMap::new();

        Self {
            config,
            state,
            committed_log,
            uncommitted_log,
            store,
            snapshot,
            nodes,
            connections,
        }
    }

    async fn add_new_node(&mut self, node_name: String, addr: SocketAddr) -> Result<()> {
        if self.nodes.contains_key(&node_name) {
            return Ok(());
        }
        self.nodes.insert(node_name, addr);
        let stream = TcpStream::connect(addr).await?;
        self.connections.insert(addr, stream);
        Ok(())
    }

    async fn commit_log(&mut self) {
        self.committed_log.append(&mut self.uncommitted_log);
        self.uncommitted_log.clear();
    }

    async fn process_entries(&mut self, leader_addr: SocketAddr, mut entries: Vec<LogEntry>) {
        self.uncommitted_log.append(&mut entries);
        if let Some(leader_stream) = self.connections.get_mut(&leader_addr) {
            let packet = Packet::from_bytes(PacketType::LogAck, Vec::new());
            let _ = Packet::send(leader_stream, packet).await;
        }
    }

    pub async fn start(&mut self) -> Result<()> {
        self.state.timeout_check().await;
        let (tx, mut rx) = mpsc::channel::<NodeEvent>(10);

        tokio::spawn(handle_discovery(self.config.clone(), tx.clone()));
        tokio::spawn(handle_messaging(self.config.clone(), tx.clone()));

        // Event loop
        loop {
            tokio::select! {
                Some(event) = rx.recv() => {
                    match event {
                        NodeEvent::NewNode(node_name, addr) => {
                            let _ = self.add_new_node(node_name, addr).await;
                        }
                        NodeEvent::ClientReq(sender, command) => {
                            println!("NodeEvent: ClientReq: {:?}", command);
                            // let _ = self.store.execute(command);
                            match self.state.get_mode() {
                                NodeMode::Follower => {
                                    // follower::send_to_leader(command);
                                }
                                NodeMode::Candidate => {
                                    // respond error to client
                                }
                                NodeMode::Leader => {
                                    if matches!(command, Command::GET {..}) {
                                        if let Some(value) = self.store.execute(command) {
                                            let _ = sender.send(value);
                                        }
                                    } else {
                                        let log_entry = LogEntry::new(command);
                                        self.uncommitted_log.push(log_entry);
                                    }
                                }
                            }
                        }
                        NodeEvent::LogAck(len, hash) => {
                            println!("NodeEvent: LogAck: len {}, hash {}", len, hash);
                            // todo: use hash to verify
                            // let len = len as usize;
                            // if len == self.uncommitted_log.len() {
                            // }
                            self.state.add_log_acks();

                            let majority_nodes = (self.connections.len()+1)/2;
                            if self.state.get_log_acks() > majority_nodes as u32 {
                                self.commit_log().await;
                                // send committed log msg
                            }
                        }
                        NodeEvent::LogCommitted => {
                            println!("NodeEvent: LogCommitted");
                            self.state.reset_timeout_timer();
                            match self.state.get_mode() {
                                NodeMode::Follower => {
                                    self.commit_log().await;
                                }
                                NodeMode::Candidate => {
                                    // do nothing
                                }
                                NodeMode::Leader => {
                                    // do nothing
                                }
                            }
                        }
                        NodeEvent::LogEntry(leader_addr, term, entries) => {
                            println!("NodeEvent: LogEntry from {} with term {}, entries len {}", leader_addr, term, entries.len());
                            self.state.reset_timeout_timer();
                            match self.state.get_mode() {
                                NodeMode::Follower => {
                                    self.process_entries(leader_addr, entries).await;
                                }
                                NodeMode::Candidate => {
                                    self.state.init_follower(term).await;
                                    self.process_entries(leader_addr, entries).await;
                                }
                                NodeMode::Leader => {
                                    if term > self.state.get_term() {
                                        self.state.init_follower(term).await;
                                        self.process_entries(leader_addr, entries).await;
                                    }
                                }
                            }
                        }
                        NodeEvent::VoteReqReceived(addr, new_term) => {
                            println!("NodeEvent: VoteReqReceived from {} with term {}", addr, new_term);
                            // check voted_term
                            if self.state.get_mode() != NodeMode::Follower || new_term <= self.state.get_term() {
                                println!("Vote request from {} rejected", addr);
                                continue;
                            }

                            if let Some(stream) = self.connections.get_mut(&addr) {
                                if follower::send_vote(stream, new_term).await.is_ok() {
                                    self.state.update_voted_term(new_term);
                                }
                            }

                            self.state.reset_timeout_timer();
                        }
                        NodeEvent::VoteReceived => {
                            println!("NodeEvent: VoteReceived");
                            if self.state.get_mode() != NodeMode::Candidate {
                                continue;
                            }

                            self.state.add_vote();
                            let majority_nodes = (self.connections.len() + 1)/2;

                            if self.state.get_votes() > majority_nodes as u32 {
                                self.state.init_leader().await;
                                leader::heartbeat(&mut self.connections, self.state.get_term(), &self.uncommitted_log).await;
                            }
                        }
                    }
                }
                _ = self.state.timeout_check() => {
                    println!("\nTimeout!\nPrevState: {:?}\nTerm: {}\nNodes: {:?}\nConnections: {:?}\n", self.state.get_mode(), self.state.get_term(), self.nodes, self.connections.len());
                    match self.state.get_mode() {
                        NodeMode::Follower => {
                            self.state.init_candidate().await;
                            println!("New State: Candidate");
                            candidate::request_votes(&mut self.connections, self.state.get_term()).await;
                        }
                        NodeMode::Candidate => {
                            let majority_nodes = (self.connections.len() + 1)/2;

                            println!("Votes: {}, Votes needed: >{}", self.state.get_votes(), majority_nodes);

                            if self.state.get_votes() <= majority_nodes as u32 {
                                println!("Election failed, restarting election");
                                self.state.init_candidate().await;
                                println!("New State: Candidate");
                                candidate::request_votes(&mut self.connections, self.state.get_term()).await;
                            } else {
                                println!("Election succeeded, becoming leader");
                                self.state.init_leader().await;
                                leader::heartbeat(&mut self.connections, self.state.get_term(), &self.uncommitted_log).await;
                            }
                        }
                        NodeMode::Leader => {
                            leader::heartbeat(&mut self.connections, self.state.get_term(), &self.uncommitted_log).await;
                            self.state.reset_timeout_timer();
                        }
                    }
                }
            }
        }
    }
}
