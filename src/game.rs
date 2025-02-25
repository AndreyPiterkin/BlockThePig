use std::collections::{HashSet, VecDeque};

use crate::board::{Board, Tile};
use crate::posn::{Position, HexPosn};
use crate::maps::ClassicMap;

/**
* Represents a instant in the Game, with a board and a Pig.
*/
pub struct GameInstance<P: Position> {
    board: Board<P>,
    pig: Box<dyn Pig<P>>,
}

/**
* A Pig is a position (more specifically, a position somewhere on the board)
* and a movement strategy with which the pig determines where to move next.
*/
pub trait Pig<P: Position> {
    fn r#move(&self, board: Board<P>) -> P;
    fn position(&self) -> P;
}

/**
* Implementation of the classic Block the Pig pig.
*/
pub struct ClassicPig {
    pub position: HexPosn,
    max_scan_dist: usize,
}

impl ClassicPig {

    /*
    * Constructs the classic pig, which starts at (5, 2) and
    * has the max view distance
    */
    pub fn new() -> ClassicPig 
    {
        ClassicPig {
            position: (5, 2).into(),
            max_scan_dist: usize::MAX,
        } 
    }
}

impl Pig<HexPosn> for ClassicPig {
    fn r#move(&self, board: Board<HexPosn>) -> HexPosn {
        let mut queue: VecDeque<HexPosn> = VecDeque::new();
        let mut seen_set: HashSet<HexPosn> = HashSet::new();
        // Approximately: level order BFS
        // Go outward until the closest exit is found, then complete the level
        // (Maybe) cache the seen set between moves
        // Collect all of the possible paths to equidistant exists (manhattan distance combinations on hex grid)
        // Sort by classical ordering (topleft > topright > left > right > bottomleft > bottomright), then move
        // (Maybe) cache this list of paths
        // On next move call, filter out paths with blocked points
        // perform BFS again, skipping past cached seen positions
        // insert new paths of same distance into the list maintaining sorted order)
        self.position
    }

    fn position(&self) -> HexPosn {
        self.position
    }
}

/**
* Implementation of the game instance for BtP.
*/
impl GameInstance<HexPosn> {

    /**
    * Creates a classic variant of the game (iOS), with a 11x5 rectangular, hexagonal-tile map,
    * with a standard pig.
    */
    pub fn classic_game() -> Self {
        GameInstance {
            board: Board::new(ClassicMap::new(11, 5)),
            pig: Box::new(ClassicPig::new())
        }
    }

    /**
    * Gets the dimensions of the board, normalized so that the game behaves as if the top-left
    * corner of the board is at (0, 0).
    * TODO: this should live in board
    */
    pub fn get_dimensions(&self) -> (usize, usize) {
        let (HexPosn { r: br, c: bc }, HexPosn { r: er, c: ec }) = self.board.get_dimensions();
        (er - br + 1, ec - bc + 1)
    }

    pub fn tile_at(&self, p: HexPosn) -> Tile {
        self.board.get_tile(p)
    }

    /**
    * Provides the position of the pig as needed for rendering.
    */
    pub fn pig_pos(&self) -> HexPosn {
        self.pig.position()
    }

    /**
    * Places a block at the given location, printing if incorrect.
    * TODO: return a result type for unpacking in the driver/referee code.
    */
    pub fn block(&mut self, p: HexPosn) -> () {
        if self.pig_pos() != p {
            self.board.place_block(p);
        } else {
            println!("Can't place the block on the pig!");
        }
    }
}
