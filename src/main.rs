use crossterm::terminal::enable_raw_mode;
// This is the game main file.
use scrabble::client::{client_work, create_client};

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    enable_raw_mode().unwrap();
    env_logger::init();

    let mut c = create_client().await.expect("Failed to create client");
    client_work(&mut c).await;
    loop {}
}
