use crate::PlayerNo;

#[allow(dead_code)]
pub struct Player {
    player_num: PlayerNo,
    total_score: u32,
    in_turn: bool,
    letter_sack: [char; 7],
}

impl Player {
    // Instantiates a new player.
    // The pool of letter sack will handled by the server and the current
    // player sack will be updated after every turn.
    // The number of characters in the sack will be 7.
    // I can use an array instead and I will keep updating the letters.
    pub fn new(p: PlayerNo, l_s: [char; 7]) -> Self {
        Player {
            player_num: p,
            total_score: 0,
            in_turn: false,
            letter_sack: l_s,
        }
    }
}
