use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut socket = TcpStream::connect("127.0.0.1:8000").await?;
    println!("Connected to the server");

    let msg = b"Hello from client!";
    socket.write_all(msg).await?;

    Ok(())
}
