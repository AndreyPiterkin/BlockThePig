use std::error::Error;
use std::result::Result;
use std::fmt as fmt;
use std::cell::RefCell;

pub struct Board {
    board: Vec<Vec<RefCell<Tile>>>,
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
    Free(FreeTile),
    Edge,
    Blocked,
}

/*
* A `FreeTile` is a tile that is empty and can be moved to.
* It has a collection of neighbors, guaranteed to be sorted 
* from top-left, moving clockwise and ending at right.
*/
#[derive(Debug, Clone)]
pub struct FreeTile {
    top_left: Option<(usize, usize)>,
    top_right: Option<(usize, usize)>,
    right: Option<(usize, usize)>,
    bot_right: Option<(usize, usize)>,
    bot_left: Option<(usize, usize)>,
    left: Option<(usize, usize)>,
}

impl FreeTile {
    fn new(tl: (usize, usize), tr: (usize, usize), r: (usize, usize),
        br: (usize, usize), bl: (usize, usize), l: (usize, usize)) -> FreeTile {
        FreeTile {
            top_left: Option::Some(tl),
            top_right: Option::Some(tr),
            right: Option::Some(r),
            bot_right: Option::Some(br),
            bot_left: Option::Some(bl),
            left: Option::Some(l),
        }
    }

    fn block_top_left(&mut self) -> () {
        self.top_left = Option::None;
    }

    fn block_top_right(&mut self) -> () {
        self.top_right = Option::None;
    }
    
    fn block_right(&mut self) -> () {
        self.right = Option::None;
    }

    fn block_bot_right(&mut self) -> () {
        self.bot_right = Option::None;
    }

    fn block_bot_left(&mut self) -> () {
        self.bot_left = Option::None;
    }

    fn block_left(&mut self) -> () {
        self.left = Option::None;
    }
}

impl Board {
    pub fn new(rows: usize, cols: usize) -> Board {
       let tiles : Vec<Vec<RefCell<Tile>>> =  (0..rows).map(|r| (0..cols).map(|c| {
            let max_row = rows - 1;
            let max_col = cols - 1;
            let t : Tile = match (r, c) {
                (0, _) => Tile::Edge,
                (_, 0) => Tile::Edge,
                (max_row, _) => Tile::Edge,
                (_, max_col) => Tile::Edge,
                _ => Tile::Free(FreeTile::new(
                    (r, c-1),
                    (r, c+1),
                    (r-1, if r%2==0 { c-1 } else { c }),
                    (r-1, if r%2==0 { c } else { c + 1}),
                    (r+1, if r%2==0 { c-1 } else { c }),
                    (r+1, if r%2==0 { c } else { c+1 }),
                )),
            };
            RefCell::new(t)
        }).collect()).collect();
        Board {
            board: tiles,
        }
    }

    pub fn get_tile(&self, (r, c): (usize, usize)) -> &RefCell<Tile> {
        if r >= self.board.len() || c >= self.board.get(r).unwrap().len() {
            panic!("Row or Col access for board is out of bounds: {:?}", (r,c));
        }
        self.board.get(r).unwrap().get(c).unwrap()
    }

    fn remove_neighbor_connections(&self, tile: &FreeTile) -> Result<(), ClosedTileError> {
        match &mut *self.get_tile(tile.top_left.ok_or(ClosedTileError)?).borrow_mut() {
            Tile::Free(ft) => { 
                ft.block_bot_right();
            },
            _ => ()
        };
        match &mut *self.get_tile(tile.top_right.ok_or(ClosedTileError)?).borrow_mut() {
            Tile::Free(ft) => { 
                ft.block_bot_right();
            }
            _ => ()
        };
        match &mut *self.get_tile(tile.right.ok_or(ClosedTileError)?).borrow_mut() {
            Tile::Free(ft) => { 
                ft.block_left();
            }
            _ => ()
        };  
        match &mut *self.get_tile(tile.bot_right.ok_or(ClosedTileError)?).borrow_mut() {
            Tile::Free(ft) => { 
                ft.block_top_left();
            }
            _ => ()
        };  
        match &mut *self.get_tile(tile.bot_left.ok_or(ClosedTileError)?).borrow_mut() {
            Tile::Free(ft) => { 
                ft.block_top_right();
            }
            _ => ()
        };  
        match &mut *self.get_tile(tile.left.ok_or(ClosedTileError)?).borrow_mut() {
            Tile::Free(ft) => { 
                ft.block_right();
            }
            _ => ()
        };  
        return Result::Ok(());
    }

    pub fn place_block(&self, r: usize, c: usize) -> () {
        let tile = self.board.get(r).unwrap().get(c).unwrap();
        if let Tile::Blocked = &*tile.borrow() {
            println!("Can't place a tile on another tile!");
            return;
        }

        match &*tile.borrow() {
            Tile::Free(ft) => {
                self.remove_neighbor_connections(&ft);
            }
            _ => ()
        }
        self.board.get(r).unwrap().get(c).unwrap().replace_with(|old| Tile::Blocked);
    }

    pub fn get_dimensions(&self) -> (usize, usize) {
        (self.board.len(), self.board.get(0).unwrap().len())
    }
}
