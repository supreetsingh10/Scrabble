use super::board::ScrabTile;
use log::debug;
use tokio::sync::Mutex;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct ServerPlayer {
    score: u32,
    player_sack: Arc<Mutex<Vec<ScrabTile>>>,
    words: Arc<Mutex<Vec<String>>>,
}

impl ServerPlayer {
    pub fn new() -> Self {
        ServerPlayer { 
            score: 0, 
            player_sack: Arc::new(Mutex::new(Vec::with_capacity(7))),
            words: Arc::new(Mutex::new(Vec::new()))
        }
    }

    pub fn describe_player(&self) {
        debug!("{:?}", self);
    }
}
