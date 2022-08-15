use crate::point::Point;
use std::convert::TryFrom;

use num_enum::{IntoPrimitive, TryFromPrimitive};
use rand_derive2::RandGen;
use serde::{Deserialize, Serialize};

use strum_macros::EnumIter;

/// Represents 8 possible directions as well as a [NONE](Self::NONE) direction.
#[derive(
    Clone,
    Copy,
    Debug,
    Ord,
    PartialOrd,
    PartialEq,
    Eq,
    Hash,
    RandGen,
    IntoPrimitive,
    TryFromPrimitive,
    EnumIter,
    Serialize,
    Deserialize,
)]
#[repr(i32)]
pub enum Direction {
    NORTH = 0,
    NORTHEAST = 1,
    EAST = 2,
    SOUTHEAST = 3,
    SOUTH = 4,
    SOUTHWEST = 5,
    WEST = 6,
    NORTHWEST = 7,
    NONE = 8,
}

impl Direction {
    pub fn diagonal(&self) -> bool {
        self.num() % 2 == 1
    }
    pub fn discrete_angle(&self, other: Direction) -> i32 {
        (other.num() - self.num()).rem_euclid(8)
    }
    pub fn num(&self) -> i32 {
        (*self).into()
    }
    pub fn rotate_cw(&self, eights: i32) -> Direction {
        match self {
            Direction::NONE => Direction::NONE,
            _ => Direction::try_from((self.num() + eights).rem_euclid(8)).unwrap(),
        }
    }
    pub fn rotate_ccw(&self, eights: i32) -> Direction {
        match self {
            Direction::NONE => Direction::NONE,
            _ => Direction::try_from((self.num() - eights).rem_euclid(8)).unwrap(),
        }
    }
    pub fn x_dir(&self) -> Direction {
        Point::new(self.x(), 0).abs_dir()
    }
    pub fn y_dir(&self) -> Direction {
        Point::new(0, self.y()).abs_dir()
    }
    pub fn x(&self) -> i32 {
        Point::from(*self).x
    }
    pub fn y(&self) -> i32 {
        Point::from(*self).y
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_east1() {
        assert_eq!(6, Direction::SOUTH.discrete_angle(Direction::EAST));
        assert_eq!(Direction::EAST, Direction::SOUTH.rotate_cw(6));
        assert_eq!(Direction::EAST, Direction::SOUTH.rotate_ccw(2));
    }
    #[test]
    fn test_east2() {
        assert_eq!(4, Direction::WEST.discrete_angle(Direction::EAST));
        assert_eq!(Direction::EAST, Direction::WEST.rotate_cw(4));
        assert_eq!(Direction::EAST, Direction::WEST.rotate_ccw(4));
    }
    #[test]
    fn test_east3() {
        assert_eq!(2, Direction::NORTH.discrete_angle(Direction::EAST));
        assert_eq!(Direction::EAST, Direction::NORTH.rotate_cw(2));
        assert_eq!(Direction::EAST, Direction::NORTH.rotate_ccw(6));
    }
    #[test]
    fn test_east4() {
        assert_eq!(0, Direction::EAST.discrete_angle(Direction::EAST));
    }

    #[test]
    fn test_ne1() {
        assert_eq!(2, Direction::NORTHWEST.discrete_angle(Direction::NORTHEAST));
    }
    #[test]
    fn test_ne2() {
        assert_eq!(4, Direction::SOUTHWEST.discrete_angle(Direction::NORTHEAST));
    }
    #[test]
    fn test_ne3() {
        assert_eq!(6, Direction::SOUTHEAST.discrete_angle(Direction::NORTHEAST));
    }
    #[test]
    fn test_ne4() {
        assert_eq!(0, Direction::NORTHEAST.discrete_angle(Direction::NORTHEAST));
    }
}
