use std::cell::Cell;
pub struct Board<'a> {
    board: Vec<Vec<Tile<'a>>>,
}

pub enum Tile<'a> {
    Free(FreeTile<'a>),
    Edge,
    Blocked,
}

/*
* A `FreeTile` is a tile that is empty and can be moved to.
* It has a collection of neighbors, guaranteed to be sorted 
* from top-left, moving clockwise and ending at right.
*/
pub struct FreeTile<'a> {
    top_left: Cell<&'a Tile<'a>>,
    top_right: Cell<&'a Tile<'a>>,
    right: Cell<&'a Tile<'a>>,
    bot_right: Cell<&'a Tile<'a>>,
    bot_left: Cell<&'a Tile<'a>>,
    left: Cell<&'a Tile<'a>>
}

impl<'a> FreeTile<'a> {
    fn new(initial_neighbor: &'a Tile<'a>) -> FreeTile<'a> {
        FreeTile {
            top_left: Cell::new(initial_neighbor),
            top_right: Cell::new(initial_neighbor),
            right: Cell::new(initial_neighbor),
            bot_right: Cell::new(initial_neighbor),
            bot_left: Cell::new(initial_neighbor),
            left: Cell::new(initial_neighbor)
        }
    }

    fn set_top_left(&self, tl: &'a Tile<'a>) -> () {
        self.top_left.set(tl);
    }

    fn set_top_right(&self, tl: &'a Tile<'a>) -> () {
        self.top_right.set(tl);
    }
    
    fn set_right(&self, tl: &'a Tile<'a>) -> () {
        self.right.set(tl);
    }

    fn set_bot_right(&self, tl: &'a Tile<'a>) -> () {
        self.bot_right.set(tl);
    }

    fn set_bot_left(&self, tl: &'a Tile<'a>) -> () {
        self.bot_left.set(tl);
    }

    fn set_left(&self, tl: &'a Tile<'a>) -> () {
        self.left.set(tl);
    }
}

impl<'a> Board<'a> {
    pub fn new(row_count: usize, col_count: usize) -> Board<'a> {
        let initializer_tile : &'a Tile<'a> = &Tile::Blocked;
        let b = Board {
            board: (0..row_count).map(|r| (0..col_count).map(|c| Tile::Free(FreeTile::new(initializer_tile))).collect()).collect()
        };
        
        return b;
    }
}
