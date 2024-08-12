use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use tokio::time::{self, Duration};
use rand::Rng;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop{
        let result = TcpStream::connect("127.0.0.1:8000").await;
        if let Ok(mut socket) = result {
            println!("Connected to the server");
            let mut interval = time::interval(Duration::from_millis(100));      //rate at which to send log messages
            let min = 1;
            let max = 1000;
            for _ in 0..1000{                               //number of log messages to send
                interval.tick().await;
                let log_message = generate_log_message(min,max);
                if let Err(e)=socket.write_all(log_message.as_bytes()).await{
                    println!("Failed to send message, reconnecting...");
                    time::sleep(Duration::from_secs(2)).await;
                    break; 
                }
                if let Err(e)=socket.write_all(b"\n").await{
                    println!("Failed to send message, reconnecting...");
                    time::sleep(Duration::from_secs(2)).await;
                    break; 
                }
            }
            
        } else {
            let e = result.unwrap_err(); // Get the error to print
            println!("Failed to connect to server: {:?}", e);
            println!("Reconnecting...");
            time::sleep(Duration::from_secs(1)).await; // Retry after 1 second
        }
    }
    Ok(())
}


fn generate_log_message(min: u32, max: u32) -> String {          //function to crates random log messages
    let mut random_num = rand::thread_rng();
    format!("Log message {}", random_num.gen_range(min..=max))
}
