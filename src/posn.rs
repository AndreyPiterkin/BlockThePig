
/* Represents a position on a hexagonal grid in 2D space.
 * Generally, up is negative, down positive, left negative, right positive.
 * But, because this is a hexagonal grid, we have 6 neighbors:
 * up-left, up-right, left, right, bot-left, bot-right.
 * The actual coords depend on the row the original one is in.
 */

use std::cmp::max;
use std::cmp::min;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::Iterator;
use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
pub struct HexPosn {
    pub r: isize,
    pub c: isize
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub enum HexDirection {
    UpLeft,
    UpRight,
    Left,
    Right,
    BotLeft,
    BotRight
}

impl PartialOrd for HexDirection {
    fn lt(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::UpLeft, Self::UpLeft) => false,
            (Self::UpLeft, _) => false,
            (_, Self::UpLeft) => true,
            (Self::UpRight, Self::UpRight) => false,
            (Self::UpRight, _) => false,
            (_, Self::UpRight) => true,
            (Self::Left, Self::Left) => false,
            (Self::Left, _) => false,
            (_, Self::Left) => true,
            (Self::Right, Self::Right) => false,
            (Self::Right, _) => false, 
            (_, Self::Right) => true, 
            (Self::BotLeft, Self::BotLeft) => false,
            (Self::BotLeft, _) => false,
            (_, Self::BotLeft) => true,
            (Self::BotRight, Self::BotRight) => false,
        }
    }

    fn le(&self, other: &Self) -> bool {
        self.lt(other) || self == other
    }

    fn gt(&self, other: &Self) -> bool {
        !self.le(other)
    }

    fn ge(&self, other: &Self) -> bool {
        !self.lt(other)
    }

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            match (self.lt(other), self == other) {
                (true, _) => Ordering::Less,
                (false, true) => Ordering::Equal,
                (false, false) => Ordering::Greater
            }
        )
    }
}

pub trait Position: Sized + Copy + Clone + PartialEq + Eq + Hash + Debug {
    type Dir;
    fn get_neighbors(&self) -> Vec<(Self::Dir, Self)>;
    fn get_bounding_coords(&self, p: Self) -> (Self, Self);
    fn get_bounds(coords: impl Iterator<Item = Self>) -> (Self, Self);
    fn in_bounds(&self, bounds: (Self, Self)) -> bool;
    fn get_neighbor(&self, d: Self::Dir) -> Self;
}

impl HexPosn {
    pub fn from_vals(r: isize, c: isize) -> Self {
        HexPosn::from((r, c))
    }
}

impl From<(isize, isize)> for HexPosn {
    fn from((r, c): (isize, isize)) -> Self {
        HexPosn {
            r,
            c
        }
    }
}

impl Into<(isize, isize)> for HexPosn {
    fn into(self) -> (isize, isize) {
        (self.r, self.c)
    }
}

impl From<(HexDirection, HexPosn)> for HexPosn {
    fn from((d, p): (HexDirection, HexPosn)) -> Self {
        p.get_neighbor(d) 
    }
}

impl Into<Vec<(HexDirection, HexPosn)>> for HexPosn {
    fn into(self) -> Vec<(HexDirection, HexPosn)> {
        self.get_neighbors()
    }
}


// TODO: do this more nicely?
impl Position for HexPosn {
    type Dir = HexDirection;

    fn get_neighbor(&self, d: Self::Dir) -> Self {
        let r = self.r;
        let c = self.c;
        let row_offset = r % 2;
        match d {
            HexDirection::UpLeft => HexPosn::from((r - 1, c - 1 + row_offset)),
            HexDirection::UpRight => HexPosn::from((r - 1, c + row_offset)),
            HexDirection::Right => HexPosn::from((r, c + 1)),
            HexDirection::BotLeft => HexPosn::from((r + 1, c - 1 + row_offset)),
            HexDirection::BotRight => HexPosn::from((r + 1, c + row_offset)),
            HexDirection::Left => HexPosn::from((r, c - 1))
        }
    }
    
    // Vec to guarantee consistent ordering
    fn get_neighbors(&self) -> Vec<(Self::Dir, Self)> {
        vec![
            HexDirection::UpLeft,
            HexDirection::UpRight,
            HexDirection::Left,
            HexDirection::Right,
            HexDirection::BotLeft,
            HexDirection::BotRight
        ].into_iter().map(|dir| (dir, self.get_neighbor(dir))).collect()
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

