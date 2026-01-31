use anyhow::Result;
use tokio::{io::AsyncWriteExt, net::TcpStream};

pub async fn send_vote(stream: &mut TcpStream) -> Result<()> {
    let vote = b"vote";
    stream.write_all(vote).await?;
    Ok(())
}

pub async fn process_entries() {}
