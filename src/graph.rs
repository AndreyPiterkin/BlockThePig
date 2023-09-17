use std::cell::Cell;

pub struct Board {
    board: Vec<Vec<Tile>>,
}

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
pub struct FreeTile {
    neighbors: Vec<Cell<Tile>>
}

impl FreeTile {
    fn new() -> FreeTile {
        FreeTile {
            neighbors: vec![],
        }
    }

    fn add_neighbor(&mut self, t: &Tile) {
        self.neighbors.push(t);
    }
}

impl Board {
    pub fn new(row_count: usize, col_count: usize) -> Board {
        let b = Board {
            board: (0..row_count).map(|r| (0..col_count).map(|c| Tile::Free(FreeTile::new())).collect()).collect()
        };
        
        return b;
    }
}
