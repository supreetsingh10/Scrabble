use std::char;

use crate::Coordinate;

#[derive(Copy, Clone, Debug)]
#[allow(dead_code)]
pub struct ScrabTile {
    point: u16,
    letter: char,
}

impl ScrabTile {
    fn default() -> Self {
        ScrabTile { point: 0, letter: ' ' }
    }
}


pub type Grid = [[ScrabTile; 15]; 15];

pub trait Grids {
    type Output;

    fn new() -> Box<Self::Output>;

    fn update_grid(cur_coords: &Coordinate, input: char); 
}

impl Grids for Grid {
    type Output = Grid;

    fn new() -> Box<Grid> {
       Box::new([[ScrabTile::default(); 15]; 15]) 
    }

    fn update_grid(cur_coord: &Coordinate, input: char) {
        // self[coord.x as usize][coord.y as usize].letter = input;

    }
}

