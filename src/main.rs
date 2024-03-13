// This is the game main file. 
use scrabble::client::{client_work, create_client};

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    env_logger::init(); 

    let mut c = create_client().await.expect("Failed to create client"); 
    client_work(&mut c).await;
    loop {}
}
