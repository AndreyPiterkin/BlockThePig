use std::collections::{HashSet, VecDeque};
use std::cmp::Ordering;

use crate::board::{Board, ClassicTile, Tile};
use crate::posn::{Position, HexPosn, HexDirection};
use crate::maps::ClassicMap;

/**
* Represents a instant in the Game, with a board and a Pig.
*/
pub struct GameInstance<P: Position, T: Tile> {
    board: Board<P, T>,
    pig: Box<dyn Pig<P, T>>,
    free_blocks: usize,
}

/**
* A Pig is a position (more specifically, a position somewhere on the board)
* and a movement strategy with which the pig determines where to move next.
*/
pub trait Pig<P: Position, T: Tile> {
    fn move_pig(&mut self, board: &Board<P, T>) -> Option<P>;
    fn position(&self) -> P;
}

/**
* Implementation of the classic Block the Pig pig.
*/
pub struct ClassicPig {
    pub position: HexPosn,
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
        } 
    }
}

impl Pig<HexPosn, ClassicTile> for ClassicPig {
    fn move_pig(&mut self, board: &Board<HexPosn, ClassicTile>) -> Option<HexPosn> {
        let mut queue: VecDeque<(HexPosn, Vec<(HexDirection, HexPosn)>)> = VecDeque::new();
        queue.push_back((self.position, vec![]));
        let mut seen_set: HashSet<HexPosn> = HashSet::new();
        let mut equidistant_exits: Vec<Vec<(HexDirection, HexPosn)>> = vec![];
        let mut found_exit = false;

        while queue.len() > 0 && !found_exit {
            let level_len = queue.len();
            for _ in 0..level_len {
                // Queue should never be empty inside a level
                let (curr_pos, moves) = queue.pop_front().unwrap(); 
                if seen_set.contains(&curr_pos) {
                    continue;
                }

                let curr_tile = board.get_tile(curr_pos);
                if curr_tile.is_some_and(|t| t.is_exit()) {
                    found_exit = true; 
                    equidistant_exits.push(moves.clone());
                    continue;
                }

                let neighbors = curr_pos.get_neighbors();
                for (d, p) in &neighbors {
                    let tile = board.get_tile(*p);
                    if tile.is_some_and(|t| t.is_passable()) && !seen_set.contains(p) {
                        let mut updated_moves = moves.clone();
                        updated_moves.push((*d, curr_pos));
                        queue.push_back((*p, updated_moves));
                    }
                }

                seen_set.insert(curr_pos);
            }
        }

        // Pig is blocked
        if equidistant_exits.is_empty() {
            return None;
        }

        let pig_move = equidistant_exits.first().unwrap().first().unwrap();
        self.position = HexPosn::from(*pig_move);
        Some(self.position)
    }

    fn position(&self) -> HexPosn {
        self.position
    }
}

/**
* Implementation of the game instance for BtP.
*/
impl GameInstance<HexPosn, ClassicTile> {

    /**
    * Creates a classic variant of the game (iOS), with a 11x5 rectangular, hexagonal-tile map,
    * with a standard pig.
    */
    pub fn classic() -> Self {
        GameInstance {
            board: Board::new(ClassicMap::new(11, 5)),
            pig: Box::new(ClassicPig::new()),
            free_blocks: 4
        }
    }

    /**
    * Gets the dimensions of the board, normalized so that the game behaves as if the top-left
    * corner of the board is at (0, 0).
    * TODO: some leakage of dimension information; also this isn't even really a sensible question,
    * since boards can have irregular shape
    */
    pub fn get_dimensions(&self) -> (isize, isize) {
        let (HexPosn { r: br, c: bc }, HexPosn { r: er, c: ec }) = self.board.get_dimensions();
        (er - br + 1, ec - bc + 1)
    }

    pub fn tile_at(&self, p: HexPosn) -> Option<ClassicTile> {
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
    * Some(false) => did not win, pig won
    * Some(true) => player won
    */
    pub fn block(&mut self, p: HexPosn) -> Result<Option<bool>, String> {
        if self.pig_pos() != p {
            let res = self.board.place(p, ClassicTile::Block);
            res.and_then(|_| {
                if self.free_blocks > 0 {
                    self.free_blocks -= 1;
                    Ok(None)
                } else {
                    match self.pig.move_pig(&self.board) {
                        Some(new_pos) => Ok(self.board.get_tile(new_pos).expect("pig ended up outside the map?").is_exit().then_some(false)),
                        None => Ok(Some(true)),
                    }
                }
            })
        } else {
            Err("Can't place a block onto the pig".to_string())
        }
    }
}
