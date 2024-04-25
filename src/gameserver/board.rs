use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::char;

use crate::Coordinate;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct ScrabTile {
    pub point: u16,
    pub letter: char,
}

impl ScrabTile {
    pub fn default() -> Self {
        ScrabTile {
            point: 0,
            letter: ' ',
        }
    }

    pub fn new(l: char, p: u16) -> Self {
        ScrabTile {
            point: p,
            letter: l,
        }
    }
}

pub type Grid = [[ScrabTile; 15]; 15];
pub type SackTiles = Vec<ScrabTile>;

pub trait Grids {
    fn new() -> Box<Self>;
    fn if_empty(&self, coords: &Coordinate) -> bool; 
}

impl Grids for Grid {
    fn new() -> Box<Grid> {
        Box::new([[ScrabTile::default(); 15]; 15])
    }

    fn if_empty(&self, coords: &Coordinate) -> bool {
        self[coords.x as usize][coords.y as usize].letter == ' '
    }
}

pub trait Sack<T> {
    fn new_sack() -> Self;
    fn shuffle_sack(&mut self);
    fn populate(&mut self, num: u16, ch: char, point: u16);
    fn get_tile(&mut self) -> Option<T>;
}

//1 point: E ×12, A ×9, I ×9, O ×8, N ×6, R ×6, T ×6, L ×4, S ×4, U ×4
//2 points: D ×4, G ×3
//3 points: B ×2, C ×2, M ×2, P ×2
//4 points: F ×2, H ×2, V ×2, W ×2, Y ×2
//5 points: K ×1
//8 points: J ×1, X ×1
//10 points: Q ×1, Z ×1
impl Sack<ScrabTile> for SackTiles {
    fn new_sack() -> SackTiles {
        let mut sack = SackTiles::new();
        sack.populate(12, 'e', 1);
        sack.populate(9, 'a', 1);
        sack.populate(9, 'i', 1);
        sack.populate(8, 'o', 1);
        sack.populate(6, 'n', 1);
        sack.populate(6, 'r', 1);
        sack.populate(6, 't', 1);
        sack.populate(4, 'l', 1);
        sack.populate(4, 's', 1);
        sack.populate(4, 'u', 1);

        sack.populate(4, 'd', 2);
        sack.populate(4, 'g', 2);

        sack.populate(2, 'b', 3);
        sack.populate(2, 'c', 3);
        sack.populate(2, 'm', 3);
        sack.populate(2, 'p', 3);

        sack.populate(2, 'f', 4);
        sack.populate(2, 'h', 4);
        sack.populate(2, 'v', 4);
        sack.populate(2, 'w', 4);
        sack.populate(2, 'y', 4);

        sack.populate(1, 'k', 5);

        sack.populate(1, 'j', 8);
        sack.populate(1, 'x', 8);

        sack.populate(1, 'q', 10);
        sack.populate(1, 'z', 10);

        sack.populate(2, '_', 0);

        sack
    }

    fn get_tile(&mut self) -> Option<ScrabTile> {
        self.pop()
    }

    // Shuffle works like this, we will iterate through the vector
    // generate a random number, switch the positions of the current number and the tile on that
    // randomly generated index.
    // do this till the end of the vector.
    fn shuffle_sack(&mut self) {
        let mut rng = rand::thread_rng();
        self.shuffle(&mut rng);
    }

    fn populate(&mut self, num: u16, ch: char, point_awared: u16) {
        (0..num).for_each(|_| {
            self.push(ScrabTile {
                point: point_awared,
                letter: ch,
            })
        })
    }
}
