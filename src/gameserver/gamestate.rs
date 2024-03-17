use crate::gameserver::players::PLAYER; 
use crate::constants::global::Coordinate;
use crate::{Action, ClientEvent}; 

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct BoardState {
    pub current_coord: Coordinate, 
    pub player_turn: PLAYER,
    pub client_event: Option<ClientEvent>,
}

impl BoardState {
    fn new() -> Self {
        BoardState { current_coord: (0,0), player_turn: PLAYER::Player1, client_event: None }
    }

    pub fn initialize() -> Box<BoardState> {
        Box::new(BoardState::new())
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
