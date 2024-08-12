use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut socket = TcpStream::connect("127.0.0.1:8000").await?;
    println!("Connected to the server");

    let log_messages = vec![
        "Log message 1",
        "Log message 2",
        "Log message 3",
    ];

    for message in log_messages {
        socket.write_all(message.as_bytes()).await?;
        socket.write_all(b"\n").await?;
    }

    Ok(())
}
