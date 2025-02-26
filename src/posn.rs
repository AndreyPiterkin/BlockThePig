
/* Represents a position on a hexagonal grid in 2D space.
 * Generally, up is negative, down positive, left negative, right positive.
 * But, because this is a hexagonal grid, we have 6 neighbors:
 * up-left, up-right, left, right, bot-left, bot-right.
 * The actual coords depend on the row the original one is in.
 */

use std::cmp::max;
use std::cmp::min;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::Iterator;
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

pub trait Position: Sized + Copy + Clone + PartialEq + Eq + Hash + Debug {
    type Dir;
    fn get_neighbors(&self) -> HashMap<Self::Dir, Self>;
    fn get_bounding_coords(&self, p: Self) -> (Self, Self);
    fn get_bounds(coords: impl Iterator<Item = Self>) -> (Self, Self);
    fn in_bounds(&self, bounds: (Self, Self)) -> bool;
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
    fn get_neighbors(&self) -> HashMap<Self::Dir, Self> {
        let r = self.r;
        let c = self.c;
        let row_offset = r % 2;
        let mut h = HashMap::new();
        h.insert(HexDirection::UpLeft, HexPosn::from((r - 1, c - 1 + row_offset)));
        h.insert(HexDirection::UpRight, HexPosn::from((r - 1, c + row_offset)));
        h.insert(HexDirection::Right, HexPosn::from((r, c + 1)));
        h.insert(HexDirection::BotLeft, HexPosn::from((r + 1, c + row_offset)));
        h.insert(HexDirection::BotRight, HexPosn::from((r + 1, c - 1 + row_offset)));
        h.insert(HexDirection::Left, HexPosn::from((r, c - 1)));

        h
    }

    fn get_bounding_coords(&self, p: Self) -> (Self, Self) {
        ((min(self.r, p.r), min(self.c, p.c)).into(), (max(self.r, p.r), max(self.c, p.c)).into())
    }

    fn get_bounds(coords: impl Iterator<Item = Self>) -> (Self, Self) {
        coords.fold(((0,0).into(), (0,0).into()), |(minp, maxp), next| {
            let (new_min, _) = minp.get_bounding_coords(next);
            let (_, new_max) = maxp.get_bounding_coords(next);
            (new_min, new_max)
        })
    }

    fn in_bounds(&self, bounds: (Self, Self)) -> bool {
        let (minb, maxb) = bounds; 
        (minb.r <= self.r && self.r <= maxb.r) && (minb.c <= self.c && self.c <= maxb.c)
    }
}

