use core::panic;
use std::process::exit;

use crate::constants::global::PORT;
use crate::Response;
use crate::{Action, ClientEvent, KeyPress, KeyboardEvent};
use log::debug;
use tokio::io::AsyncReadExt;
use tokio::{io::AsyncWriteExt, net::TcpStream};

// create a client and bind to the given port.
pub async fn create_client(ip: Option<&String>) -> std::io::Result<TcpStream> {
    match ip {
        Some(addr) => {
            let add  = format!(":{}", PORT); 
            let w = addr.to_owned();
            TcpStream::connect(w + add.as_str()).await
        },
        None => {
            TcpStream::connect(format!("0.0.0.0:{}", PORT)).await
        }
    }
}

// make then async, muliple keypresses are going here over time.
// Implement a channel where the client and server can communincate.
pub async fn client_update(client: &mut TcpStream) {
    let mut keyboard_event = KeyboardEvent::new();

    loop {
        // Use the keypresses to stream the data.
        let c_event = match keyboard_event.next().await {
            Some(keypress) => match keypress {
                KeyPress::Key(k) => ClientEvent::from_key(&k),
                KeyPress::TICK => ClientEvent {
                    action: Action::WAITING,
                },
                KeyPress::ERROR => panic!("Error came check events"),
                KeyPress::NONE => ClientEvent {
                    action: Action::NONE,
                },
            },
            None => ClientEvent {
                action: Action::NONE,
            },
        };

        // if player sack is empty then fill it up with values.
        match c_event.action {
            Action::QUIT => {
                let val = serde_json::to_string(&c_event).expect("Failed to serialize");
                if let Err(e) = client.write_all(val.trim().as_bytes()).await {
                    panic!(
                        "Failed to write the stream, server will not cut off Error: {}",
                        e
                    );
                }
                exit(0);
            }
            Action::WRITE(w) => {
                //TODO
                // Delete the character w from the player sack.
                let val = serde_json::to_string(&c_event).expect("Failed to serialize");

                if let Err(e) = client.write_all(val.trim().as_bytes()).await {
                    panic!(
                        "Failed to write the stream, server will not cut off Error: {}",
                        e
                    );
                }

                let resp = match server_response(client).await {
                    Ok(r) => r,
                    Err(e) => panic!("Error occured {}", e),
                };

                // Update the score on the client.
            }
            Action::DIRECTION(m) => {
                let dir =
                    serde_json::to_string(&c_event).expect("Failed to serialize the direction");

                if let Err(e) = client.write_all(dir.trim().as_bytes()).await {
                    panic!("Failed to write the movement stream, Error : {}", e);
                }

                let resp = match server_response(client).await {
                    Ok(r) => r,
                    Err(e) => panic!("Server response fn call failed, Error {}", e),
                };

                // update the current visuals on the screen related to movement of the cursor.
            },
            Action::END => {
                let end_round = serde_json::to_string(&c_event).expect("Failed to serialize the end"); 
                if let Err(e) = client.write_all(end_round.trim().as_bytes()).await {
                    panic!("Failed to end the round, Error : {}", e);
                }

                let resp = match server_response(client).await {
                    Ok(r) => r,
                    Err(e) => panic!("Failed to end the round {}", e),
                };

                // Update the round and make it available for another player. 
            }
            _ => {
                // None and waiting are left, they do nothing for now. Hence do not change it.
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

    println!(
        "Server respone. {}",
        String::from_utf8_lossy(buffer[0..buf_cursor].as_ref())
    );

    serde_json::from_slice(&buffer[0..buf_cursor])
}
