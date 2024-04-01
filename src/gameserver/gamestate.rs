use crate::{gameserver::board::Grid, Action, ClientEvent, Coordinate, Response, PLAYER};
use std::borrow::BorrowMut;
use tokio::sync::Mutex;
use std::sync::Arc;

use super::{board::SackTiles, server_player::ServerPlayer};

#[derive(Debug, Clone)]
pub struct BoardState {
    current_coord: Coordinate,
    // Player enum is client facing. 
    // I need a player struct for the server as well which will take care of the sack, the
    // characters the player uses. 
    player_turn: PLAYER,
    client_event: Option<ClientEvent>,
    scrab_sack: Arc<Mutex<Option<SackTiles>>>,
    scrab_grid: Arc<Mutex<Option<Grid>>>,
    players: Arc<Mutex<Vec<ServerPlayer>>>,
}

// Who ever makes the server will be the player one.
impl BoardState {
    fn new() -> Self {
        BoardState {
            current_coord: Coordinate::new(),
            player_turn: PLAYER::Player1,
            client_event: None,
            scrab_sack: Arc::new(Mutex::new(None)),
            scrab_grid: Arc::new(Mutex::new(None)),
            players: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn get_current_coord(&self) -> &Coordinate {
        &self.current_coord
    }

    pub fn update_scrab_grid(&mut self, resp: &Response) {
        // board state will have updated values. use those values to update the characters.
    }

    pub async fn add_player(&self, player: ServerPlayer) {
        let mut l = self.players.lock().await;
        l.push(player);
    }

    pub async fn describe_players(&self) {
        self.players.lock().await.iter().for_each(|f| {
            f.describe_player(); 
        });
    }

    pub fn get_current_coord_mut(&mut self) -> &mut Coordinate {
        self.current_coord.borrow_mut()
    }

    pub fn set_client_event(&mut self, c_event: Option<ClientEvent>) {
        self.client_event = c_event;
    }

    pub fn get_client_event(&self) -> Option<ClientEvent> {
        self.client_event
    }

    pub fn initialize() -> Box<BoardState> {
        Box::new(BoardState::new())
    }

    pub async fn set_scrab_grid(&self, grid: Grid) {
        *self.scrab_grid.lock().await = Some(grid);
    }

    pub fn get_action(&self) -> Action {
        match &self.client_event {
            Some(c_event) => {
                // Here we will get the action
                c_event.action.to_owned()
            }
            None => {
                // Return none if No action.
                Action::NONE
            }
        }
    }
}
