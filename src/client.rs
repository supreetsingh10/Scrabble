use std::process::exit;

use crate::constants::global::PORT;
use crate::{Action, ClientEvent, KeyPress, KeyboardEvent};
// use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use log::debug;
use tokio::io::AsyncReadExt;
use tokio::{io::AsyncWriteExt, net::TcpStream};

// create a client and bind to the given port.
pub async fn create_client() -> std::io::Result<TcpStream> {
    TcpStream::connect(format!("0.0.0.0:{}", PORT)).await
}

// make then async, muliple keypresses are going here over time.
// Implement a channel where the client and server can communincate. 
pub async fn client_update(client: &mut TcpStream) {
    loop {
        // Use the keypresses to stream the data.
        if let Some(ce) = keypresses().await {
            if ce.action == Action::QUIT {
                exit(0);
            }

            let val = serde_json::to_string(&ce).expect("Failed to serialize");
            let _ = client.write_all(val.trim().as_bytes()).await;
            server_response(client).await;
        }

        // this is where the client will process the server response and render the changes. 
        let _ = client.flush().await;
    }
}

async fn server_response(stream: &mut TcpStream) {
    let mut buffer = [0u8; 2048];
    let mut buf_cursor = 0;

    debug!("We are on the client side and looking for response from the server");
    match stream.read(&mut buffer).await {
        Ok(n) => {
            debug!("{} bytes read", n);
        },
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

    println!("{}", String::from_utf8_lossy(buffer[0..buf_cursor].as_ref()));
}

// the read function has to be made async.
// use the mpsc to check if the channel thing will work for keypresses.
pub async fn keypresses() -> Option<ClientEvent> {
    let mut keyboard_event = KeyboardEvent::new();
    let (rx, mut tx) = tokio::sync::mpsc::channel::<ClientEvent>(50);
    tokio::spawn(async move {
        loop {
            if let KeyPress::Key(w) = keyboard_event.next().await.unwrap() {
                rx.send(ClientEvent::from_key(&w)).await.expect("Failed");
            }
        }
    });

    tx.recv().await
}
