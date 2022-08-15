use crate::direction::Direction;
use core::fmt;
use serde::*;
use std::cmp::{max, Ordering};
use std::convert::{From, TryFrom};
use std::ops;

/// 2D (grid) point with integer coordinates [x](Self::x) and [y](Self::y). It has member functions for many common
/// operations such as computing the [manhattan distance](Self::manhattan_distance) as well as operator implementations for easy arithmetic.
#[derive(Clone, Copy, Debug, Ord, PartialOrd, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
    /// Compares this to another point and gives a direction from this to the other.
    pub fn dir_obj(&self, other: &Point) -> Direction {
        match self.x.cmp(&other.x) {
            Ordering::Greater => match self.y.cmp(&other.y) {
                Ordering::Less => Direction::NORTHWEST,
                Ordering::Equal => Direction::WEST,
                Ordering::Greater => Direction::SOUTHWEST,
            },
            Ordering::Equal => match self.y.cmp(&other.y) {
                Ordering::Less => Direction::NORTH,
                Ordering::Equal => Direction::NONE,
                Ordering::Greater => Direction::SOUTH,
            },
            Ordering::Less => match self.y.cmp(&other.y) {
                Ordering::Less => Direction::NORTHEAST,
                Ordering::Equal => Direction::EAST,
                Ordering::Greater => Direction::SOUTHEAST,
            },
        }
    }

    /// Like [dir_obj](Self::dir_obj) but directly translates the [Direction] back to a point which
    /// can be used like a delta.
    pub fn dir(&self, other: &Point) -> Point {
        Point::from(self.dir_obj(other))
    }

    /// Gives the direction in which the given point is as seen from the origin.
    pub fn abs_dir(&self) -> Direction {
        Point::new(0, 0).dir_obj(&self)
    }

    /// L-1 norm. As a grid-pathfinding heuristic it represents number of moves on a uniform cost 4-connected grid.
    pub fn manhattan_distance(&self, other: &Point) -> i32 {
        (other.x - self.x).abs() + (other.y - self.y).abs()
    }

    /// L-inf norm. As a grid-pathfinding heuristic it represents number of moves on a uniform cost 8-connected grid.
    pub fn move_distance(&self, other: &Point) -> i32 {
        max((other.x - self.x).abs(), (other.y - self.y).abs())
    }

    /// L-2 norm.
    pub fn euclidean_distance(&self, other: &Point) -> f32 {
        ((other.x - self.x).pow(2) as f32 + (other.y - self.y).pow(2) as f32).sqrt()
    }

    /// Neighbours on a 4-connected grid.
    pub fn neumann_neighborhood(&self) -> Vec<Point> {
        vec![
            Point::new(self.x + 1, self.y),
            Point::new(self.x, self.y + 1),
            Point::new(self.x - 1, self.y),
            Point::new(self.x, self.y - 1),
        ]
    }

    /// Retrieves a single neighbour on an 8-connected grid, with the indexing used being similar
    /// to that in [Direction].
    pub fn moore_neighbor(&self, dir_num: i32) -> Point {
        *self + Direction::try_from(dir_num.rem_euclid(8)).unwrap()
    }

    /// Neighbours on an 8-connected grid.
    pub fn moore_neighborhood(&self) -> Vec<Point> {
        vec![
            Point::new(self.x, self.y + 1),
            Point::new(self.x + 1, self.y + 1),
            Point::new(self.x + 1, self.y),
            Point::new(self.x + 1, self.y - 1),
            Point::new(self.x, self.y - 1),
            Point::new(self.x - 1, self.y - 1),
            Point::new(self.x - 1, self.y),
            Point::new(self.x - 1, self.y + 1),
        ]
    }

    /// Alternative neighborhood, takes a square of a given size centered around self.
    pub fn general_moore_neighborhood(&self, size: i32) -> Vec<Point> {
        let mut neigh = vec![];
        for x in self.x - size..=(self.x + size) {
            for y in self.y - size..=(self.y + size) {
                if x != self.x || y != self.y {
                    neigh.push(Point::new(x, y));
                }
            }
        }
        neigh
    }
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point({},{})", self.x, self.y)
    }
}
impl From<Direction> for Point {
    fn from(val: Direction) -> Self {
        match val {
            Direction::NORTH => Point::new(0, 1),
            Direction::NORTHEAST => Point::new(1, 1),
            Direction::EAST => Point::new(1, 0),
            Direction::SOUTHEAST => Point::new(1, -1),
            Direction::SOUTH => Point::new(0, -1),
            Direction::SOUTHWEST => Point::new(-1, -1),
            Direction::WEST => Point::new(-1, 0),
            Direction::NORTHWEST => Point::new(-1, 1),
            Direction::NONE => Point::new(0, 0),
        }
    }
}

impl ops::Mul<i32> for Point {
    type Output = Point;

    fn mul(self, rhs: i32) -> Point {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Add<Direction> for Point {
    type Output = Point;
    fn add(self, rhs: Direction) -> Point {
        self + Point::from(rhs)
    }
}

impl ops::Neg for Point {
    type Output = Point;
    fn neg(self) -> Point {
        Point {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl ops::Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Point {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
