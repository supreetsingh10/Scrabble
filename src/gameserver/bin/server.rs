// This is the server main file.
use log::{debug, info};
use scrabble::{constants::global::PORT, Action};
use scrabble::ClientEvent;
use std::net::SocketAddr;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};
use serde::{Serialize, Deserialize};
use scrabble::gameserver::gamestate::BoardState; 

type Connection = (TcpStream, SocketAddr);
#[tokio::main]
async fn main() {
    // we will need to initialize the scrabble board state. 
    // Scrabble board state will include the values related to the values entered on the board. 
    env_logger::init();
    println!("Running the server");
    let listener = TcpListener::bind(format!("0.0.0.0:{}", PORT))
        .await
        .expect("Failed to bind to the port");

    let mut board_state = BoardState::initialize();

    tokio::spawn(async move {
        match listener.accept().await {
            Ok(mut connection) => {
                debug!("Accepted connection {:?}", connection);
                tokio::spawn(async move {
                    loop {
                        handle_connection(&mut connection, &mut board_state).await;
                    }
                });
            }
            Err(e) => {
                println!("Failed to accept the client because {}", e);
            }
        }
    });

    loop {}
}

trait ActionServer {
    fn process_action(&self); 
}

impl ActionServer for Action {
    fn process_action(&self) {
    }
}


#[derive(Serialize, Deserialize, Debug)]
struct Response {
    response: String,
}

// this is the place where we have the client event. 
fn request_handler(board_state: &mut Box<BoardState>) -> Response {
    board_state.get_action();
    Response { response: String::from("Hello world") }
}

// here the buffer is completely populating the 1024 bytes hence when it finds a null character it
// crashes.
async fn handle_connection(con: &mut Connection, board_state: &mut Box<BoardState>) {
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
        board_state.client_event = Some(c_event); 
        let resp = request_handler(board_state);
        stream.flush().await.unwrap();
        stream.write(resp.response.as_bytes()).await.unwrap();
        // I will clean up the stream an write on it. 
        // stream.flush().await; 
    }

}
