use crossterm::terminal::enable_raw_mode;
// This is the game main file.
use scrabble::client::{client_update, create_client};

#[tokio::main]
// we need to use clap to make commands for this. 
// server will have to be instantiated. 
// player will have to be created. 
async fn main() {
    println!("Hello, world!");
    enable_raw_mode().unwrap();
    env_logger::init();

    let mut stream = create_client().await.expect("Failed to create client");
    // client work has a forever loop. 
    client_update(&mut stream).await;
}
