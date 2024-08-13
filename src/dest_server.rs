use tokio::net::TcpListener;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {    //asynchronous functions enables concurrent programming by returning future context
    let listener = TcpListener::bind("127.0.0.1:9000").await?;   //? is similar to match { Ok()=> , Err()=>) consciecely
    println!("Server listening on 127.0.0.1:9000");

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("New client connected: {:?}", addr);   //:? is debug notation for traits which derives display and debug traits

        tokio::spawn(async move {   //creates an asynchronous task that runs concurrently
            let mut buffer = [0u8; 1024];   //mutable since we going to update the buffer areay

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
                        println!("Failed to read from socket; err = {:?}", e);
                        break;
                    }
                }
            }
        });
    }
}
