use super::board::ScrabTile;
use log::{debug, info};
use std::collections::LinkedList;
use std::sync::Arc;
use tokio::sync::Mutex;

const MAX_SACK: u16 = 7;

#[derive(Clone, Debug)]
pub struct ServerPlayer {
    score: u32,
    // The tiles of the player that are available to him.
    player_sack: Arc<Mutex<LinkedList<ScrabTile>>>,
    words: Arc<Mutex<Vec<String>>>,
}

impl ServerPlayer {
    pub fn new() -> Self {
        ServerPlayer {
            score: 0,
            player_sack: Arc::new(Mutex::new(LinkedList::new())),
            words: Arc::new(Mutex::new(Vec::new())),
        }
    }


    // the tiles that have to be added.
    pub async fn lacking_tiles(&self) -> u16 {
        (MAX_SACK as u16) - (self.player_sack.lock().await.len() as u16)
    }

    // TODO how to pop this from the linkedlist. 
    pub async fn find_tile(&self, ch: char) -> bool {
        let ac = Arc::clone(&self.player_sack);
        let mut pop: usize = 0; 
        for (c, i) in self.player_sack.lock().await.iter().enumerate() {
            if i.letter == ch {
                info!("FOUNDDDDD");
                pop = c; 
            }
        }


        return true;
    }

    // fill sack
    pub async fn fill_sack(&self, tile: ScrabTile) {
        Arc::clone(&self.player_sack).lock().await.push_back(tile);
        // self.player_sack.lock().await.push_back(tile);
    }

    // Takes the tile from the player sack.
    pub async fn how_many_tile(&self) -> usize {
        self.player_sack.lock().await.len()
    }

    pub fn describe_player(&self) {
        debug!("{:?}", self);
    }
}
