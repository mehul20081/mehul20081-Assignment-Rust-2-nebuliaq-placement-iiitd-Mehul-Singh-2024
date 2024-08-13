use std::collections::VecDeque;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::time::{self, Duration};

use elasticsearch::{Elasticsearch, BulkParts};
use serde_json::json;
use std::error::Error;

async fn index_logs_bulk(elastic_client: &Elasticsearch, bulk_messages: Vec<String>) -> Result<(), Box<dyn Error>> {   //since we dont know the size of error at compile time, we return a pointer to it in heap which takes usize.
    let mut body: Vec<String> = Vec::new();  //need mutable reference since we update it
    for i in bulk_messages{
        body.push(json!({
            "index": { "_index": "logs" }    //we construct the required input format of body
        }).to_string()+"\n" + &i) 
    }

    let mut retries = 3;   //number of times, we'll try to index logs to elastiseach, configuarable
    for i in 0..retries{
        let response = elastic_client
        .bulk(BulkParts::Index("logs"))
        .body(body.clone())    // we clone body since its in a loop n one iteration would have taken up ownership not allowing other iterations to use it
        .send()
        .await?;
        if !response.status_code().is_success() {
            println!("Failed to index logs: {:?}", response.text().await?);
        }
        else{
            println!("SUCCESS!!!");
            return Ok(());
        }
    }
    Ok(())
}

async fn fwd_msg(mut client_socket: TcpStream, addr : SocketAddr, dest_addr: &str,elastic_client: Elasticsearch) -> Result<(), Box<dyn std::error::Error>> {
    let mut dest_socket = TcpStream::connect(dest_addr).await?;
    let mut buffer = [0; 1024];   //buffer is an array of size 1024 containing 0 at each place
    let mut message_batch = VecDeque::with_capacity(100);   // we define capacity as 100 since afther tbat we need to flush the buffer
    let mut interval = time::interval(Duration::from_secs(10));   //interval here is the time at which we periodically flush the underfilled buffer

    loop {
        tokio::select! {
            result = client_socket.read(&mut buffer) => {   //we try to read from the client socket into the buffer
                match result {
                    Ok(n) if n == 0 => {   
                        println!("Client disconnected: {:?}", addr);
                        break;
                    }
                    Ok(n) => {   //on success, read() returns the size of the read object
                        let message = String::from_utf8_lossy(&buffer[..n]).to_string();   //since we stored msg in bytes in buffer, we typecast it to String, since buffer time is unknown at compile time, we use a ptr to buffer instead which is const usize.
                        message_batch.push_back(message); // Adding the message to the batch

                        if message_batch.len() >= 100 {
                            let batch: Vec<String> = message_batch.drain(..).collect();  //we use drain(..) so we dont need to copy or create new obj, non primitime types dont implement Copy()
                            let batched_messages = batch.join("\n");
                            dest_socket.write_all(batched_messages.as_bytes()).await?;
                            if let Err(e) = index_logs_bulk(&elastic_client, batch).await {    //calls function to index logs in elastisearch, takes Vector of String logs as parameter.
                                println!("Error indexing logs in elastisearch: {:?}", e);
                            }
 // Clear the batch after sending
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to read from socket; err = {:?}", e);
                        break;
                    }
                }
            }
            
            _ = interval.tick() => {     // this runs when 10s is up and buffer length <100
                if !message_batch.is_empty() {   //also works in case of a client disconnect and residual buffer

                    let batch: Vec<String> = message_batch.drain(..).collect();
                    let batched_messages = batch.join("\n");
                    dest_socket.write_all(batched_messages.as_bytes()).await?;

                    if let Err(e) = index_logs_bulk(&elastic_client,batch).await {
                        println!("Error indexing logs in elastisearch: {:?}", e);
                    }

                }
            }
        }
    }

    if !message_batch.is_empty() {
        let batch: Vec<String> = message_batch.drain(..).collect();
        let batched_messages = batch.join("\n");;
        dest_socket.write_all(batched_messages.as_bytes()).await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8000").await?;
    println!("Server listening on 127.0.0.1:8000");

    let elastic_client = Elasticsearch::default(); //initialising the elastisearch client
    let dest_addr = "127.0.0.1:9000";   //destination server

    loop {
        let (client_socket, addr) = listener.accept().await?;   //BLOCKING call till client connects
        println!("New client connected: {:?}", addr);

        let elastic_client = elastic_client.clone();  // we clone to avoid oenership and lifetime issues
        let dest_addr = dest_addr.clone();

        tokio::spawn(async move {

            if let Err(e) = fwd_msg(client_socket,addr, &dest_addr,elastic_client).await {  //fwd_msg function is called in asynchronous context with a blocking call
                println!("Error handling client: {:?}", e);
            }
        });
    }
}
