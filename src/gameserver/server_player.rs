use super::board::ScrabTile;
use log::debug;
use tokio::sync::Mutex;
use std::sync::Arc;
use std::collections::LinkedList; 

const MAX_SACK: u16 = 7;

#[derive(Clone, Debug)]
pub struct ServerPlayer {
    score: u32,
    player_sack: Arc<Mutex<LinkedList<ScrabTile>>>,
    words: Arc<Mutex<Vec<String>>>,
}

impl ServerPlayer {
    pub fn new() -> Self {
        ServerPlayer { 
            score: 0, 
            player_sack: Arc::new(Mutex::new(LinkedList::new())),
            words: Arc::new(Mutex::new(Vec::new()))
        }
    }

    pub async fn lacking_tiles(&self) -> u16 {
        (MAX_SACK as u16) - (self.player_sack.lock().await.len() as u16)
    }

    pub async fn take_tile(&self, tile: ScrabTile) {
        self.player_sack.lock().await.push_back(tile);
    }

    pub async fn find_tile(&self, fl: char) -> bool {
        // Check design wise if we should take in a character or a srabtile. 
        // A character makes more sense because the 
        // self.player_sack.lock().await.iter().file

        false
    }

    pub fn describe_player(&self) {
        debug!("{:?}", self);
    }
}
