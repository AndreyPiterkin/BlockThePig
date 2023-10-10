use crate::board::{Board, Tile};
use crate::maps::ClassicMap;

/**
* Represents a instant in the Game, with a board and a Pig.
*/
pub struct GameInstance {
    board: Board,
    pig: Pig,
}

/**
* A Pig is a position (more specifically, a position somewhere on the board)
* and a strategy with which the pig determines where to move next.
*/
pub struct Pig {
    position: (usize, usize),
    strategy: Box<dyn Fn((usize, usize)) -> (usize, usize)>,
}


/**
* Implementation of the pig.
*/
impl Pig {

    /**
    * Given an initial position and move function, produces a new pig at that location and with
    * that strategy.
    */
    fn new<F>((r, c): (usize, usize), f: F) -> Pig 
    where
        F: Fn((usize, usize)) -> (usize, usize) + 'static,
    {
        Pig {
            position: (r, c),
            strategy: Box::new(f),
        } 
    }

    /**
    * Creates a standard pig (one that starts at (5, 2) on the standard map, 
    * and uses the standard movement strategy (based on the iOS Mobile variant).
    */
    fn standard() -> Pig {
        Pig::new((5, 2), |(r, c)| (r, c))
    }

    /**
    * Get the position of the pig for rendering and computation.
    */
    pub fn get_pos(&self) -> (usize, usize) {
        self.position
    }
}

/**
* Implementation of the game instance for BtP.
*/
impl GameInstance {

    /**
    * Creates a classic variant of the game (iOS), with a 11x5 rectangular, hexagonal-tile map,
    * with a standard pig.
    */
    pub fn classic_game() -> GameInstance {
        GameInstance {
            board: Board::new(ClassicMap::new(11, 5)),
            pig: Pig::standard(),
        }
    }

    /**
    * Gets the dimensions of the board, normalized so that the game behaves as if the top-left
    * corner of the board is at (0, 0).
    */
    pub fn get_dimensions(&self) -> (usize, usize) {
        let ((br, bc), (er, ec)) = self.board.get_dimensions();
        return (er - br, ec - bc)
    }

    /**
    * Gets the tile at the row and the column. Since the game abstracts over the actual coordinate
    * bounds of the board, treats the given row and column as being in that abstracted space,
    * translating as needed.
    */
    pub fn tile_at(&self, (r,c): (usize, usize)) -> Tile {
        let ((br, bc), _) = self.board.get_dimensions();
        let (ir, ic) = (r + br, c + bc);
        self.board.get_tile((ir, ic)).clone()
    }

    /**
    * Provides the position of the pig as needed for rendering.
    */
    pub fn pig_pos(&self) -> (usize, usize) {
        self.pig.get_pos()
    }

    /**
    * Places a block at the given location, printing if incorrect.
    * TODO: return a result type for unpacking in the driver/referee code.
    */
    pub fn block(&mut self, (r, c): (usize, usize)) -> () {
        if self.pig_pos() != (r,c) {
            self.board.place_block(r, c);
        } else {
            println!("Can't place the block on the pig!");
        }
    }
}
