use crate::board::{Board, Tile};

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
            board: Board::new(11, 5),
            pig: Pig::standard(),
        }
    }

    pub fn get_dimensions(&self) -> (usize, usize) {
        self.board.get_dimensions()
    }

    // TODO: Replace tile-at with some rendering logic in here to avoid for loop in main
    pub fn tile_at(&self, (r,c): (usize, usize)) -> Tile {
        self.board.get_tile((r, c)).clone().into_inner()
    }

    pub fn pig_pos(&self) -> (usize, usize) {
        self.pig.get_pos()
    }

    pub fn block(&mut self, (r, c): (usize, usize)) -> () {
        self.board.place_block(r, c);
    }
}
