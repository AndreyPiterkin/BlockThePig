use crate::board::Tile;

/**
* Represents a classic, rectangular map for BtP. Can be constructed with more than 11 rows and 5
* columns. A map is both a struct and an iterator that produces all of the tiles in the map.
* This is done so that any shape of map can be created; potentially INFINITE maps could exist,
* though that is a far stretch goal, and unclear how it should be implemented.
*/
pub struct ClassicMap {
    r: usize,
    c: usize,
    curr_r: usize,
    curr_c: usize,
}

/**
* Classic map implementation.
*/
impl ClassicMap {
    /**
    * Given a row size and column size, create a new classic map.
    */
    pub fn new(r: usize, c: usize) -> ClassicMap {
        ClassicMap { r, c, curr_r: 0, curr_c: 0 }
    }
}

/**
* Because maps in this implementation of BtP are iterators that produce the tiles,
* we must implement that trait for the classic map.
*/
impl Iterator for ClassicMap {
    type Item = ((usize, usize), Tile);

    /**
    * Produces the next tile-position pair in the sequence, or None if there are none left.
    * For ClassicMap, produces a list of tiles such that edges of the "rectangular" map are Edge
    * Tiles, meaning the pig can escape, with every other tile being a Free Tile. Returns None when
    * we have produced all tiles.
    * TODO: Populate map with initial generation of blocked tiles.
    */
    fn next(&mut self) -> Option<Self::Item> {
        let ret: Option<Self::Item> = match (self.curr_r, self.curr_c) {
            (r, c) if (r >= self.r || c >= self.c) => None,
            (r, c) if (r == self.r - 1 || c == self.c - 1) => Some(((r, c), Tile::Edge)),
            (r, c) if (r == 0 || c == 0) => Some(((r, c), Tile::Edge)),
            (r, c) => Some(((r, c), Tile::Free)),
        };

        if self.curr_c >= self.c - 1 {
            self.curr_c = 0;
            self.curr_r += 1;
        } else {
            self.curr_c += 1;
        }
        ret
    }
}
