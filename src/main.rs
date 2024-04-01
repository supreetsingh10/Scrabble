use crossterm::terminal::enable_raw_mode;
use scrabble::client::{client_update, create_client};


#[tokio::main]
async fn main() {
    println!("Hello, world!");
    enable_raw_mode().unwrap();
    env_logger::init();

    let matches = clap::Command::new("scrabble")
        .version("1.0")
        .arg(clap::Arg::new("connect")); 


    let mut stream = create_client(matches.get_matches().get_one::<String>("connect")).await.expect("Failed to create client");
    // client work has a forever loop.
    client_update(&mut stream).await;
}
