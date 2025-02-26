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
pub struct Board<P: Position, T: Tile> {
    board: HashMap<P, T>,
}


pub trait Tile: Sized + Clone + Copy {
    // Can this tile be walked on by the pig
    fn is_passable(&self) -> bool;
    // Place the other tile onto this one, returning the resulting tile
    fn place_onto(&self, other: Self) -> Result<Self, String>;
    // TODO: further info, like tile actions?
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
pub enum ClassicTile {
    Free,
    Edge,
    Block,
}

impl Tile for ClassicTile {
    fn is_passable(&self) -> bool {
        match self {
            ClassicTile::Block => false,
            _ => true
        }
    }

    fn place_onto(&self, other: Self) -> Result<Self, String> {
       match (self, other) {
            (ClassicTile::Edge | ClassicTile::Free, ClassicTile::Block) => Ok(ClassicTile::Block),
            _ => Err("Can't place a block over a block".to_string())
       } 
    }
}

/**
* Generic implementation of the board for Block the Pig.
*/
impl<P: Position, T: Tile> Board<P, T> {

    /**
    * Takes in an iterator that produces position-tile pairs, and populates
    * the board with them. Takes at most MAX_BOARD_SIZE tiles.
    */
    pub fn new(blocks: impl Iterator<Item = (P, T)>) -> Self {
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
    pub fn get_tile(&self, posn: P) -> Option<T> {
        // TODO: maybe a bounds check? 
        self.board.get(&posn).cloned()
    }

    /**
    * Places a block at the given position if it can be placed
    */
    pub fn place(&mut self, posn: P, tile: T) -> Result<(), String> {
        self.get_tile(posn)
            .ok_or("Can't place a tile outside of the map".to_string())
            .and_then(|curr_tile| curr_tile.place_onto(tile))
            .and_then(|new_tile| {
                self.board.insert(posn, new_tile);
                Ok(())
            })
    }

    /**
    * Returns two positions that bound the entire playable gamespace, inclusive on both bounds
    */
    pub fn get_dimensions(&self) -> (P, P) {
        P::get_bounds(self.board.keys().into_iter().cloned())
    }
}
