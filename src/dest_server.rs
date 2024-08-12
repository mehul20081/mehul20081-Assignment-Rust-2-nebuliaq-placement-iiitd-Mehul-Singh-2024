use tokio::net::TcpListener;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:9000").await?;
    println!("Server listening on 127.0.0.1:9000");

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("New client connected: {:?}", addr);

        tokio::spawn(async move {
            let mut buffer = [0u8; 1024];

            loop {
                match socket.read(&mut buffer).await {
                    Ok(n) if n == 0 => {
                        println!("Client disconnected: {:?}", addr);
                        break;
                    }
                    Ok(n) => {
                        println!("Received from {}: {}", addr, String::from_utf8_lossy(&buffer[..n]));
                    }
                    Err(e) => {
                        eprintln!("Failed to read from socket; err = {:?}", e);
                        break;
                    }
                }
            }
        });
    }
}
