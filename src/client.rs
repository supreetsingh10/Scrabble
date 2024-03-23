use core::panic;
use std::process::exit;

use crate::constants::global::{DEBUG_LEVEL_1, PORT};
use crate::{Action, ClientEvent, KeyPress, KeyboardEvent};
// use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use log::{debug, info};
use tokio::io::AsyncReadExt;
use tokio::{io::AsyncWriteExt, net::TcpStream};
use crate::Response;

// create a client and bind to the given port.
pub async fn create_client() -> std::io::Result<TcpStream> {
    TcpStream::connect(format!("0.0.0.0:{}", PORT)).await
}

// make then async, muliple keypresses are going here over time.
// Implement a channel where the client and server can communincate. 
pub async fn client_update(client: &mut TcpStream) {
    let mut keyboard_event = KeyboardEvent::new();
    // let (rx, mut tx) = tokio::sync::mpsc::unbounded_channel::<ClientEvent>();

    loop {
        // Use the keypresses to stream the data.
        let c_event = match keyboard_event.next().await {
            Some(keypress) => {
                match keypress {
                    KeyPress::Key(k) => {
                        ClientEvent::from_key(&k)
                    },
                    KeyPress::NONE => ClientEvent { action: Action::NONE },
                    KeyPress::TICK => ClientEvent { action: Action::WAITING },
                    KeyPress::ERROR => panic!("Error came check events"),
                }
            }
            None => {
                    ClientEvent { action: Action::NONE }
            }
        };


        // if player sack is empty then fill it up with values.
        match c_event.action {
            Action::NONE => {

            },
            Action::WAITING => {

            },
            Action::QUIT => {
                let val = serde_json::to_string(&c_event).expect("Failed to serialize");
                if let Err(e) = client.write_all(val.trim().as_bytes()).await {
                    panic!("Failed to write the stream, server will not cut off Error: {}", e);
                }
                exit(0);
            },
            Action::WRITE(w) => {
                let val = serde_json::to_string(&c_event).expect("Failed to serialize");

                if let Err(e) = client.write_all(val.trim().as_bytes()).await {
                    panic!("Failed to write the stream, server will not cut off Error: {}", e);
                }

                let resp = match server_response(client).await {
                    Ok(r) => r,
                    Err(e) => panic!("Error occured {}", e),
                };
            },
            Action::DIRECTION(m) => {

            }
        };

        let _ = client.flush().await;
    }

}

async fn server_response(stream: &mut TcpStream) -> Result<Response, serde_json::Error> {
    let mut buffer = [0u8; 2048];
    let mut buf_cursor = 0;

    debug!("We are on the client side and looking for response from the server");
    let _ = stream.read(&mut buffer).await; 

    for (buf_sz, b_ch) in buffer.iter().enumerate() {
        if *b_ch == b'\0' {
            buf_cursor = buf_sz;
            break;
        }
    }

    println!("Server respone. {}", String::from_utf8_lossy(buffer[0..buf_cursor].as_ref()));

    serde_json::from_slice(&buffer)
}

