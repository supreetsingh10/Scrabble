use crate::{gameserver::board::Grid, Action, ClientEvent, Coordinate, PlayerNo, Response};
use log::debug;
use std::{borrow::BorrowMut, future::IntoFuture};
use std::sync::Arc;
use tokio::sync::Mutex;

use super::{board::SackTiles, server_player::ServerPlayer};

#[derive(Debug, Clone)]
pub struct BoardState {
    current_coord: Coordinate,
    // Player enum is client facing.
    // I need a player struct for the server as well which will take care of the sack, the
    // characters the player uses.
    player_turn: Arc<Mutex<PlayerNo>>,
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
            player_turn: Arc::new(Mutex::new(0)),
            client_event: None,
            scrab_sack: Arc::new(Mutex::new(None)),
            scrab_grid: Arc::new(Mutex::new(None)),
            players: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn initialize() -> Box<BoardState> {
        Box::new(BoardState::new())
    }

    pub fn get_current_coord(&self) -> &Coordinate {
        &self.current_coord
    }

    pub async fn set_scrab_sack(&self, sack: SackTiles) {
        *self.scrab_sack.lock().await = Some(sack)
    }

    pub fn update_scrab_grid(&mut self, resp: &Response) {
        // board state will have updated values. use those values to update the characters.
    }

    // will be used to upate the player sack after every game round.
    pub async fn fill_sack(&self, player: &ServerPlayer) {
        let sack = Arc::clone(&self.scrab_sack);
        let mut v = sack.lock().await;
        let v = v.as_mut().unwrap();
        for _ in 0..player.lacking_tiles().await {
            let t = v.pop().take();
            player.fill_sack(t.unwrap()).await;
        }
    }

    pub async fn get_current_player(&self) -> ServerPlayer {
        let pl = Arc::clone(&self.players).lock().await;
        let n = self.get_current_turn().await.to_owned();
        let pl = *pl; 
        pl[n as usize]
    }

    async fn get_total_players(&self) -> usize {
        let p = self.players.lock().await;
        p.len()
    }

    pub async fn add_player(&self, player: ServerPlayer) {
        // populate the sack here itself.
        let l = self.scrab_sack.lock().await.as_ref().unwrap().len();
        self.fill_sack(&player).await;
        self.players.lock().await.push(player);
        assert_eq!(l, self.scrab_sack.lock().await.as_ref().unwrap().len() + 7);
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

    pub async fn set_scrab_grid(&self, grid: Grid) {
        *self.scrab_grid.lock().await = Some(grid);
    }

    pub async fn get_current_turn(&self) -> PlayerNo {
        *self.player_turn.lock().await
    }

    // the index of the players will be incremented and then it will be looped.
    pub async fn set_next_turn(&self) {
        let turn = Arc::clone(&self.player_turn);
        let total_players = self.get_total_players().await;

        if *turn.lock().await == (total_players - 1) as u32 {
            debug!("back to player 1");
            *turn.lock().await = 0;
        } else {
            debug!("Next player");
            *turn.lock().await += 1;
        }
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
