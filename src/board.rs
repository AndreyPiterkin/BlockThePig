pub struct Board {
    board: Vec<Vec<Tile>>,
}

#[derive(Debug)]
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
#[derive(Debug)]
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
    pub fn new(row_count: usize, col_count: usize) -> Board {
       let tiles : Vec<Vec<Tile>> =  (0..row_count).map(|r| (0..col_count).map(|c| {
            match (r, c) {
                (0, _) => Tile::Edge,
                (_, 0) => Tile::Edge,
                _ => Tile::Free(FreeTile::new(
                    (r, c-1),
                    (r, c+1),
                    (r-1, if r%2==0 { c-1 } else { c }),
                    (r-1, if r%2==0 { c } else { c + 1}),
                    (r+1, if r%2==0 { c-1 } else { c }),
                    (r+1, if r%2==0 { c } else { c+1 }),
                )),
            }
        }).collect()).collect();
        Board {
            board: tiles,
        }
    }


}
