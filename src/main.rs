// This is the game main file. 
use scrabble::client::{client_work, create_client, keypresses};

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let mut c = create_client().await.expect("Failed to create client"); 
    client_work(&mut c).await;
    loop {}
}
