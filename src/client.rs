use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use tokio::time::{self, Duration};
use rand::Rng;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut socket = TcpStream::connect("127.0.0.1:8000").await?;
    println!("Connected to the server");

    let mut interval = time::interval(Duration::from_secs(1));
    let min = 1;
    let max = 1000;
    for _ in 0..100{                               //number of log messages to send
        interval.tick().await;
        let log_message = generate_log_message(min,max);
        socket.write_all(log_message.as_bytes()).await?;
        socket.write_all(b"\n").await?;
    }

    Ok(())
}
fn generate_log_message(min: u32, max: u32) -> String {          //function to crates random log messages
    let mut random_num = rand::thread_rng();
    format!("Log message {}", random_num.gen_range(min..=max))
}
