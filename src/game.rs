use crate::board::{Board, Tile};
use crate::maps::ClassicMap;

pub struct GameInstance {
    board: Board,
    pig: Pig,
}

pub struct Pig {
    position: (usize, usize),
    strategy: Box<dyn Fn((usize, usize)) -> (usize, usize)>,
}

impl Pig {
    fn new<F>((r, c): (usize, usize), f: F) -> Pig 
    where
        F: Fn((usize, usize)) -> (usize, usize) + 'static,
    {
        Pig {
            position: (r, c),
            strategy: Box::new(f),
        } 
    }

    fn standard() -> Pig {
        Pig::new((5, 2), |(r, c)| (r, c))
    }

    pub fn get_pos(&self) -> (usize, usize) {
        self.position
    }
}

impl GameInstance {
    pub fn classic_game() -> GameInstance {
        GameInstance {
            board: Board::new(ClassicMap::new(11, 5)),
            pig: Pig::standard(),
        }
    }

    pub fn get_dimensions(&self) -> (usize, usize) {
        let ((br, bc), (er, ec)) = self.board.get_dimensions();
        return (er - br, ec - bc)
    }

    pub fn tile_at(&self, (r,c): (usize, usize)) -> Tile {
        let ((br, bc), _) = self.board.get_dimensions();
        let (ir, ic) = (r + br, c + bc);
        self.board.get_tile((ir, ic)).clone()
    }

    pub fn pig_pos(&self) -> (usize, usize) {
        self.pig.get_pos()
    }

    pub fn block(&mut self, (r, c): (usize, usize)) -> () {
        if self.pig_pos() != (r,c) {
            self.board.place_block(r, c);
        } else {
            println!("Can't place the block on the pig!");
        }
    }
}
