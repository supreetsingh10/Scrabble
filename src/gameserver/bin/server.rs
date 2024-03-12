// This is the server main file. 
use tokio::{io::AsyncReadExt, net::{TcpListener, TcpStream}}; 
use std::net::SocketAddr;
use scrabble::constants::global::{PORT, ClientEvent, Action}; 

type Connection = (TcpStream, SocketAddr);
#[tokio::main]
async fn main() {
    println!("Running the server");
    let listener = TcpListener::bind(format!("0.0.0.0:{}",PORT)).await.expect("Failed to bind to the port");

    tokio::spawn(async move {
        match listener.accept().await {
            Ok(mut connection) => 
            {
                tokio::spawn(async move {
                    loop {
                        handle_connection(&mut connection).await;
                    }
                });
            }, 
            Err(e) => {
                println!("Failed to accept the client because {}", e); 
            }
        }
    });

    loop {}
}

async fn handle_connection(con: &mut Connection) {
    let (stream, _) = con; 
    let mut buffer = [0u8; 1024]; 
    match stream.read(&mut buffer).await {
        Ok(t) =>  {
            if t != 0 {
                let client_event = match serde_json::from_slice::<ClientEvent>(&mut buffer) {
                    Ok(c) => c,
                    Err(e) => {
                        panic!("Failed to deserialize {}", e)
                    }
                };
                println!("Server baby{:?}", client_event);
            }
        },
        Err(e) => {
            println!("Failed to read the buffer {}", e);
        }
    }
}
