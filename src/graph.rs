use std::cell::Cell;
use std::rc::{Weak, Rc};
pub struct Board<'a> {
    board: Vec<Vec<Weak<Tile<'a>>>>,
}

#[derive(Debug)]
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
#[derive(Debug)]
pub struct FreeTile<'a> {
    top_left: Weak<Cell<&'a Tile<'a>>>,
    top_right: Weak<Cell<&'a Tile<'a>>>,
    right: Weak<Cell<&'a Tile<'a>>>,
    bot_right: Weak<Cell<&'a Tile<'a>>>,
    bot_left: Weak<Cell<&'a Tile<'a>>>,
    left: Weak<Cell<&'a Tile<'a>>>,
}

impl<'a> FreeTile<'a> {
    fn new(initial_neighbor: &'a Tile<'a>) -> FreeTile<'a> {
        FreeTile {
            top_left: Rc::downgrade(&Rc::new(Cell::new(initial_neighbor))),
            top_right: Rc::downgrade(&Rc::new(Cell::new(initial_neighbor))),
            right: Rc::downgrade(&Rc::new(Cell::new(initial_neighbor))),
            bot_right: Rc::downgrade(&Rc::new(Cell::new(initial_neighbor))),
            bot_left: Rc::downgrade(&Rc::new(Cell::new(initial_neighbor))),
            left: Rc::downgrade(&Rc::new(Cell::new(initial_neighbor)))
        }
    }

    fn set_top_left(&self, tl: &'a Tile<'a>) -> () {
        self.top_left.upgrade().unwrap().set(tl);
    }

    fn set_top_right(&self, tl: &'a Tile<'a>) -> () {
        self.top_right.upgrade().unwrap().set(tl);
    }
    
    fn set_right(&self, tl: &'a Tile<'a>) -> () {
        self.right.upgrade().unwrap().set(tl);
    }

    fn set_bot_right(&self, tl: &'a Tile<'a>) -> () {
        self.bot_right.upgrade().unwrap().set(tl);
    }

    fn set_bot_left(&self, tl: &'a Tile<'a>) -> () {
        self.bot_left.upgrade().unwrap().set(tl);
    }

    fn set_left(&self, tl: &'a Tile<'a>) -> () {
        self.left.upgrade().unwrap().set(tl);
    }
}

impl<'a> Board<'a> {
    pub fn new(row_count: usize, col_count: usize) -> Board<'a> {
        let blocked_init_tile : &'a Tile<'a> = &Tile::Blocked;
        let edge_init_tile: &'a Tile<'a> = &Tile::Edge;
        let tiles : Vec<Vec<Weak<Tile<'a>>>> = (0..row_count).map(|r| (0..col_count).map(|c| {
            let init_tile = match (r, c) {
                (0, _) => edge_init_tile,
                (_, 0) => edge_init_tile,
                _ => blocked_init_tile,
            };
            return Rc::downgrade(&Rc::new(Tile::Free(FreeTile::new(init_tile))));
        }).collect()).collect();
        for r in 0..row_count {
            for c in 0..col_count {
                let tile: Rc<Tile<'a>> = tiles.get(r).unwrap().get(c).unwrap().upgrade().unwrap();
                match Rc::try_unwrap(tile) {
                    Result::Ok(Tile::Edge) => (),
                    Result::Ok(Tile::Blocked) => (),
                    Result::Ok(Tile::Free(freeTile)) => {
                        let row: &Vec<Weak<Tile<'a>>> = tiles.get(r).unwrap();
                        let tile: &Weak<Tile<'a>> = row.get(c).unwrap();
                        freeTile.set_left(Rc::try_unwrap(tile.upgrade().unwrap()).unwrap());
                        //freeTile.set_left(tiles.get(r).unwrap().get(c-1).unwrap());
                    }
                    _ => ()
                }
            } 
        }
        Board {
            board: tiles,
        }
    }
}
