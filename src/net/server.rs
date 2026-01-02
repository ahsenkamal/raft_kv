use crate::net::Packet;
use std::io::Result;
use std::net::{TcpListener, TcpStream};
use std::thread;

pub fn setup_server(port: u16) -> Result<()> {
    let listener = TcpListener::bind(("0.0.0.0", port))?;

    for stream in listener.incoming() {
        let stream = stream?;
        thread::spawn(|| handle_connection(stream));
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    loop {
        let packet = match Packet::from_stream(&mut stream) {
            Ok(p) => p,
            Err(_) => {
                break;
            }
        };

        handle_packet(packet);
    }
}

fn handle_packet(packet: Packet) {
    // handle packet
}
