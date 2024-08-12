use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;

async fn fwd_msg(mut client_socket: TcpStream, addr : SocketAddr, dest_addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut dest_socket = TcpStream::connect(dest_addr).await?;
    let mut buffer = [0; 1024];
    let mut message_batch = Vec::with_capacity(100);

    loop {
        match client_socket.read(&mut buffer).await {
            Ok(n) if n == 0 => {
                println!("Client disconnected: {:?}", addr);
                break;
            }
            Ok(n) => {
                let message = String::from_utf8_lossy(&buffer[..n]).to_string();
                //println!("Received from {}: {}", addr, message);

                message_batch.push(message); //adding the msg to the batch

                if message_batch.len() >= 100 {
                    let batched_messages = message_batch.join("\n");
                    dest_socket.write_all(batched_messages.as_bytes()).await?;
                    message_batch.clear(); // Clear the batch after sending
                }
            }
            Err(e) => {
                eprintln!("Failed to read from socket; err = {:?}", e);
                break;
            }
        }
    }

    if !message_batch.is_empty() {
        let batched_messages = message_batch.join("\n");
        dest_socket.write_all(batched_messages.as_bytes()).await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8000").await?;
    println!("Server listening on 127.0.0.1:8000");

    let dest_addr = "127.0.0.1:9000";

    loop {
        let (client_socket, addr) = listener.accept().await?;
        println!("New client connected: {:?}", addr);

        let dest_addr = dest_addr;

        tokio::spawn(async move {

            if let Err(e) = fwd_msg(client_socket,addr, &dest_addr).await {
                println!("Error handling client: {:?}", e);
            }
        });
    }
}
