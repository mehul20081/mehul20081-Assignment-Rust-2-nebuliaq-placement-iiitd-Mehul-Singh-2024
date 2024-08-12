use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use tokio::time::{self, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut socket = TcpStream::connect("127.0.0.1:8000").await?;
    println!("Connected to the server");

    let log_messages = vec![
        "Log message 1",
        "Log message 2",
        "Log message 3",
        "Log message 4",
        "Log message 5",
    ];

    let mut interval = time::interval(Duration::from_secs(1));

    for message in log_messages {
        interval.tick().await; 
        socket.write_all(message.as_bytes()).await?;
        socket.write_all(b"\n").await?;
    }

    Ok(())
}
