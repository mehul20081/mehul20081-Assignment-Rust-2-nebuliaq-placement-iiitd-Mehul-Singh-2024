use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use tokio::time::{self, Duration};
use rand::Rng;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {      //return type helps in error handling by either returning a value or unit tuple or an error, we use box to allocate size on heap since we dont know size of the error on compile time
    loop{
        let result = TcpStream::connect("127.0.0.1:8000").await;    //waiting call to connect to the server port
        if let Ok(mut socket) = result {   //using if let we destructure the connection for further processing and error handling
            println!("Connected to the server");   //we need to make the socket mutable since we are using it to write in a loop and dont want to take ownership
            let mut interval = time::interval(Duration::from_secs(1));      //rate at which to send log messages
            for _ in 0..1000{                               //number of log messages to send
                interval.tick().await;   // we make the client wait for the specified waiting time
                let log_message = generate_log_message();
                if let Err(_e)=socket.write_all(log_message.as_bytes()).await{   //write_all method that byte as input, strings are also seauence if bytes
                    println!("Failed to send message, reconnecting...");
                    time::sleep(Duration::from_secs(2)).await;
                    break; 
                }
                if let Err(_e)=socket.write_all(b"\n").await{      
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
    Ok(())  //need to return Result type
}


fn generate_log_message() -> String {          //function to create random log messages
    let logs=[
        "Info: Application started successfully.",
        "Debug: Debugging application.",
        "Warning: Disk space is low.",
        "Error: Failed to connect to the database.",
        "Critical: Application crashed unexpectedly.",
    ];
    let mut random_num = rand::thread_rng();  //uses randamozation
    match logs.choose(&mut random_num){
        Some(x)=>{return x.to_string();}
        None=>{return "Error2: Failed to choose a log.".to_string();}
    };
}
