use std::net::SocketAddr;

use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;

async fn fwd_msg(mut client_socket: TcpStream, addr : SocketAddr, dest_addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut dest_socket = TcpStream::connect(dest_addr).await?;
    let mut buffer = [0; 1024];

    loop {
        match client_socket.read(&mut buffer).await {
            Ok(n) if n == 0 => {
                println!("Client disconnected: {:?}", addr);
                break;
            }
            Ok(n) => {
                println!("Received from {}: {}", addr, String::from_utf8_lossy(&buffer[..n]));
                dest_socket.write_all(&buffer[..n]).await?;

            }
            Err(e) => {
                eprintln!("Failed to read from socket; err = {:?}", e);
                break;
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8000").await?;
    println!("Server listening on 127.0.0.1:8000");

    let dest_addr = "127.0.0.1:9000";

    loop {
        let (mut client_socket, addr) = listener.accept().await?;
        println!("New client connected: {:?}", addr);

        let dest_addr = dest_addr.clone();

        tokio::spawn(async move {

            if let Err(e) = fwd_msg(client_socket,addr, &dest_addr).await {
                println!("Error handling client: {:?}", e);
            }
        });
    }
}
