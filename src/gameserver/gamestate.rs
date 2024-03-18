use std::borrow::BorrowMut;

use crate::{Coordinate, gameserver::{players::PLAYER, board::Grid}, Action, ClientEvent};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct BoardState {
    current_coord: Coordinate, 
    player_turn: PLAYER,
    client_event: Option<ClientEvent>,
    scrab_grid: Option<Grid>,
}

impl BoardState {
    fn new() -> Self {
        BoardState { 
            current_coord: Coordinate::new(),
            player_turn: PLAYER::Player1, 
            client_event: None, 
            scrab_grid: None
        }
    }

    pub fn get_current_coord(&self) -> &Coordinate {
        &self.current_coord
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

    pub fn set_scrab_grid(&mut self, grid:Grid) {
        self.scrab_grid = Some(grid);
    }

    pub fn get_scrab_grid(&self) -> Option<Grid> {
        self.scrab_grid.clone()
    }

    pub fn get_action(&self) -> Action {
        match &self.client_event {
            Some(c_event) => {
                // Here we will get the action 
                c_event.action.to_owned()
            }, 
            None => {
                // Return none if No action. 
                Action::NONE
            }
        }
    }

}
