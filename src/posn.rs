
/* Represents a position on a hexagonal grid in 2D space.
 * Generally, up is negative, down positive, left negative, right positive.
 * But, because this is a hexagonal grid, we have 6 neighbors:
 * up-left, up-right, left, right, bot-left, bot-right.
 * The actual coords depend on the row the original one is in.
 */

use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
pub struct HexPosn {
    pub r: usize,
    pub c: usize
}

#[derive(Hash, PartialEq, Eq)]
pub enum HexDirection {
    UpLeft,
    UpRight,
    Left,
    Right,
    BotLeft,
    BotRight
}

pub trait Position {
    type Dir;
    fn get_neighbors(&self) -> HashMap<Self::Dir, Box<Self>>;
}

impl HexPosn {
    pub fn from_vals(r: usize, c: usize) -> Self {
        HexPosn::from((r, c))
    }
}

impl From<(usize, usize)> for HexPosn {
    fn from((r, c): (usize, usize)) -> Self {
        HexPosn {
            r,
            c
        }
    }
}

impl Into<(usize, usize)> for HexPosn {
    fn into(self) -> (usize, usize) {
        (self.r, self.c)
    }
}


// TODO: do this more nicely?
impl Position for HexPosn {
    type Dir = HexDirection;
    fn get_neighbors(&self) -> HashMap<HexDirection, Box<Self>> {
        let r = self.r;
        let c = self.c;
        let row_offset = r % 2;
        let mut h = HashMap::new();
        h.insert(HexDirection::UpLeft, Box::new(HexPosn::from((r - 1, c - 1 + row_offset))));
        h.insert(HexDirection::UpRight, Box::new(HexPosn::from((r - 1, c + row_offset))));
        h.insert(HexDirection::Right, Box::new(HexPosn::from((r, c + 1))));
        h.insert(HexDirection::BotLeft, Box::new(HexPosn::from((r + 1, c + row_offset))));
        h.insert(HexDirection::BotRight, Box::new(HexPosn::from((r + 1, c - 1 + row_offset))));
        h.insert(HexDirection::Left, Box::new(HexPosn::from((r, c - 1))));

        h
    }
}
