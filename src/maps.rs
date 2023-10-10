use crate::board::Tile;
pub struct ClassicMap {
    r: usize,
    c: usize,
    curr_r: usize,
    curr_c: usize,
}

impl ClassicMap {
    pub fn new(r: usize, c: usize) -> ClassicMap {
        ClassicMap { r, c, curr_r: 0, curr_c: 0 }
    }
}


impl Iterator for ClassicMap {
    type Item = ((usize, usize), Tile);


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
