use tokio::{io::AsyncWriteExt, net::TcpStream};
// use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers}; 
use crate::constants::global::{ClientEvent, KeyPress, KeyboardEvent, PORT};

// create a client and bind to the given port. 
pub async fn create_client() -> std::io::Result<TcpStream> {
    TcpStream::connect(format!("0.0.0.0:{}",PORT)).await
}


// make then async, muliple keypresses are going here over time. 
pub async fn client_work(client: &mut TcpStream) 
{
    loop {
        // Use the keypresses to stream the data. 
        if let Some(ce) = keypresses().await
        {
            println!("In client work loop {:?}", ce);
            // let val = serde_json::to_string(&ce).expect("Failed to serialize");
            // let _ = client.write_all(val.as_bytes()).await;
        }
        let _ = client.flush().await;
    }
}

// the read function has to be made async. 
// use the mpsc to check if the channel thing will work for keypresses. 
pub async fn keypresses() -> Option<ClientEvent> {
    let mut keyboard_event = KeyboardEvent::new(); 
    let (rx, mut tx) = tokio::sync::mpsc::channel::<ClientEvent>(50);
    tokio::spawn(async move {
        loop 
        {
            if let KeyPress::Key(w) = keyboard_event.next().await.unwrap() {
                rx.send(ClientEvent::from_key(&w)).await.expect("Failed");
            }
        }
    });

    tx.recv().await
}

