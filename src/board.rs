use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;

use crate::posn::{Position, HexPosn};


/**
 * Given that our board can take in potentially infinite iterators,
 * we define a constant which limits the number of tiles to an arbitrarily large value.
 * TODO: refactor this away, allowing for truly infinite boards
 */
const MAX_BOARD_SIZE: usize = 100;

/**
* The standard representation of a Block the Pig Board. It is a 
* Map from positions (usize, usize) to Tiles at those positions.
*/
pub struct Board<P: Position> {
    board: HashMap<P, Tile>,
}


/**
* A Tile is an enumeration of all the possible tiles at any position;
* currently, a Tile is either:
*   - Free, meaning the pig can move there AND NOT escape
*   - Edge, meaning the pig can move there AND can escape
*   - Blocked, meaning the pig can't move there.
* Edge is used as a method for defining exit routes for more complex 
* shaped boards, where exits may not be the edges of the standard 11x5
* rectangular board.
*/
#[derive(Debug, Clone, Copy)]
pub enum Tile {
    Free,
    Edge,
    Blocked,
}

/**
* Implementation of the board for Block the Pig.
*/
impl Board<HexPosn> {

    /**
    * Takes in an iterator that produces position-tile pairs, and populates
    * the board with them. Takes at most MAX_BOARD_SIZE tiles.
    */
    pub fn new(blocks: impl Iterator<Item = (HexPosn, Tile)>) -> Board<HexPosn> {
        Board { 
            board: blocks.take(MAX_BOARD_SIZE).collect()
        }
    }

    /**
    * Given a position (row column pair), return a reference to the tile at that position.
    * If the given row column pair does not exist in the board, the board will panic.
    * I have chosen to make it a panic instead of a Result type for short term convenience; this is
    * subject to change.
    */
    pub fn get_tile(&self, posn: HexPosn) -> Tile {
        if self.board.contains_key(&posn) {
            return *self.board.get(&posn).unwrap();
        } 
        panic!("posn does not exist in map: {:?}", posn);
    }

    /**
    * Places a block at the given position, unless there is already a Block there.
    */
    pub fn place_block(&mut self, posn: HexPosn) -> () {
        if self.board.contains_key(&posn) {
            if let Some(Tile::Blocked) = self.board.get(&posn) {
                println!("Can't place a tile on another tile!");
            } else {
                self.board.insert(posn, Tile::Blocked);
            }
        }
    }

    /**
    * Returns the bounds of the board; that is, returns a pair of the top-left position
    * and the bottom right position, where top and left are negative, and bottom and right are
    * positive (i.e. graphics coordinates).
    * TODO: implement this in HexPosn, to hide details of `usize`
    */
    pub fn get_dimensions(&self) -> (HexPosn, HexPosn) {
        let mut row_max: usize = std::usize::MIN;
        let mut col_max: usize = std::usize::MIN;
        let mut row_min: usize = std::usize::MAX;
        let mut col_min: usize = std::usize::MAX;
        for HexPosn { r, c } in self.board.keys() {
            row_max = max(*r, row_max);
            col_max = max(*c, col_max);
            row_min = min(*r, row_min);
            col_min = min(*c, col_min);
        } 
        ((row_min, col_min).into(), (row_max, col_max).into())
    }
}
