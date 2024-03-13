// This is the server main file. 
use log::{debug, info};
use tokio::{io::AsyncReadExt, net::{TcpListener, TcpStream}}; 
use std::{char, net::SocketAddr};
use scrabble::constants::global::{PORT, ClientEvent, Action}; 

type Connection = (TcpStream, SocketAddr);
#[tokio::main]
async fn main() 
{
    env_logger::init(); 
    println!("Running the server");
    let listener = TcpListener::bind(format!("0.0.0.0:{}",PORT)).await.expect("Failed to bind to the port");

    tokio::spawn(async move {
        match listener.accept().await {
            Ok(mut connection) => 
            {
                debug!("Accepted connection {:?}", connection);
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

fn client_req(c_event: &ClientEvent) 
{
    println!("In the sever {:?}", c_event);
}

// here the buffer is completely populating the 1024 bytes hence when it finds a null character it
// crashes.  
async fn handle_connection(con: &mut Connection) 
{
    debug!("handle connection {:?}", con);
    let (stream, _) = con; 
    let mut buffer = [0u8; 1024]; 
    let mut buf_cursor = 0; 


    match stream.read(&mut buffer).await {
        Ok(n) => {
            debug!("{} bytes read", n);
        }
        Err(_) => {
            debug!("Failed to read");
        }
    }

    for (buf_sz, b_ch) in buffer.iter().enumerate() {
        if b_ch.to_owned() == b'\0' {
            buf_cursor = buf_sz;
            debug!("Broke the loop at {}", buf_cursor);
            break;
        }
    }

    debug!("String {}", String::from_utf8_lossy(&buffer[0..buf_cursor]));
    if let Ok(c_event) = serde_json::from_slice::<ClientEvent>(&buffer[0..buf_cursor]) {
        client_req(&c_event);
    }
}
