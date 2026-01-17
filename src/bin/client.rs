use std::{env, io, net::{SocketAddr, TcpStream}};
use anyhow::{Result, anyhow};
use raft_kv::primitives::command::{Command};

fn repl(stream: &mut TcpStream) -> Result<(), anyhow::Error> {
    let mut input = String::new();
    loop {
        print!("> ");
        io::stdin().read_line(&mut input).expect("Faileld to take input");
        if input.eq_ignore_ascii_case("exit") {
            break;
        }
        let command: Command = Command::parse(&input)?;
        Command::send(stream, command)?;
    }

    Ok(())
}

fn main() -> Result<()>{
    println!("client");
    let args: Vec<String> = env::args().collect();
    let gateway_addr: SocketAddr = match args.get(1) {
        Some(s) => {
            s.parse()?
        }
        None => {
            return Err(anyhow!("Invalid address"));
        }
    };
    let mut stream = TcpStream::connect(gateway_addr)?;
    
    repl(&mut stream);

    Ok(())
}