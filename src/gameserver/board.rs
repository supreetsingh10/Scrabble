use std::char;

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


pub type Grid = [[ScrabTile; 15]; 15];

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

