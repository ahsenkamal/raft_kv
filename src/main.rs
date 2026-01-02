use std::io;
use std::sync::{Arc, Mutex};
use std::{collections::HashMap, net::TcpListener};

mod net;
use net::server::setup_server;

fn main() {
    let STORE: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let port: u16 = input.trim().parse().unwrap();

    setup_server(port);
}
