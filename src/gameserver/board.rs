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
pub type PlayerTile = ScrabTile; 
pub type SackTiles;

pub trait Grids {
    type Output;

    fn new() -> Box<Self::Output>;
}

impl Grids for Grid {
    type Output = Grid;

    fn new() -> Box<Grid> {
       Box::new([[ScrabTile::default(); 15]; 15]) 
    }
}

impl Grids for SackTiles {
    type Output = SackTiles;
    fn new() -> Box<SackTiles> {

    }
}
