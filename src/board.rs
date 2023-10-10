use std::error::Error;
use std::cmp::max;
use std::cmp::min;
use std::fmt as fmt;
use std::collections::HashMap;

pub struct Board {
    board: HashMap<(usize, usize), Tile>,
}

#[derive(Debug)]
struct ClosedTileError;
impl Error for ClosedTileError {}
impl fmt::Display for ClosedTileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tried to access fields for a Closed Tile.")
    }
}

#[derive(Debug, Clone)]
pub enum Tile {
    Free,
    Edge,
    Blocked,
}

impl Board {
    pub fn new(blocks: impl Iterator<Item = ((usize, usize), Tile)>) -> Board {
        Board { 
            board: blocks.take(100).collect()
        }
    }

    pub fn get_tile(&self, (r, c): (usize, usize)) -> &Tile {
        if self.board.contains_key(&(r, c)) {
            return self.board.get(&(r, c)).unwrap();
        } 
        panic!("Row or Col access for board is out of bounds: {:?}", (r,c));
    }

    pub fn place_block(&mut self, r: usize, c: usize) -> () {
        if self.board.contains_key(&(r, c)) {
            if let Some(Tile::Blocked) = self.board.get(&(r,c)) {
                println!("Can't place a tile on another tile!");
                return;
            }
        }

        self.board.insert((r,c), Tile::Blocked);
    }

    pub fn get_dimensions(&self) -> ((usize, usize), (usize, usize)) {
        let mut row_max: usize = std::usize::MIN;
        let mut col_max: usize = std::usize::MIN;
        let mut row_min: usize = std::usize::MAX;
        let mut col_min: usize = std::usize::MAX;
        for (r, c) in self.board.keys() {
            row_max = max(*r, row_max);
            col_max = max(*c, col_max);
            row_min = min(*r, row_min);
            col_min = min(*c, col_min);
        } 
        ((row_min, col_min), (row_max, col_max))
    }
}
