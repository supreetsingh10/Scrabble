use std::char;
use crate::constants::global::Coordinate;

#[allow(dead_code)]

#[derive(Copy, Clone, Debug)]
pub struct ScrabTile {
    point: u16,
    letter: char,
}

impl ScrabTile {
    fn default() -> Self {
        ScrabTile { point: 0, letter: ' ' }
    }
}


type Grid = [[ScrabTile; 15]; 15];

pub trait Grids {
    type Output;

    fn initialize(&self) -> Box<Self::Output>;
}

impl Grids for Grid {
    type Output = Grid;

    fn initialize(&self) -> Box<Grid> {
       Box::new([[ScrabTile::default(); 15]; 15]) 
    }
}
