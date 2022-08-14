use crate::point::Point;
use crate::rect::Rect;
use serde::{Deserialize, Serialize};

/// The [Grid] trait abstracts over containers of [Clone] and [Copy] items laid out in a rectangle
/// with a certain [width](Self::width) and [height](Self::height).
pub trait Grid<T: Clone + Copy> {
    // Static method signature; `Self` refers to the implementor type.
    fn new(width: usize, height: usize, default_value: T) -> Self;
    fn get(&self, x: usize, y: usize) -> T;
    fn get_point(&self, point: Point) -> T {
        self.get(point.x as usize, point.y as usize)
    }
    fn set_point(&mut self, point: Point, value: T) {
        self.set(point.x as usize, point.y as usize, value);
    }
    fn set(&mut self, x: usize, y: usize, value: T);
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn get_ix(&self, x: usize, y: usize) -> usize {
        x + y * self.width()
    }
    fn point_in_bounds(&self, point: Point) -> bool {
        point.x >= 0
            && point.y >= 0
            && point.x < self.width() as i32
            && point.y < self.height() as i32
    }
    fn index_in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width() && y < self.height()
    }
    fn set_rectangle(&mut self, rect: &Rect, value: T) {
        for x in rect.x1..rect.x2 {
            for y in rect.y1..rect.y2 {
                self.set(x as usize, y as usize, value);
            }
        }
    }
    fn rect(&self) -> Rect {
        Rect::new(0, 0, self.width() as i32, self.height() as i32)
    }

    fn get_rect(&self, rect: Rect) -> Vec<T> {
        rect.points_in()
            .into_iter()
            .map(|p| self.get_point(p))
            .collect::<Vec<T>>()
    }
}

/// Optimized (binary) implementation of a [Grid] of [bool]'s.
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct BoolGrid {
    pub width: usize,
    pub height: usize,
    pub values: Vec<u64>,
}

impl Grid<bool> for BoolGrid {
    fn new(width: usize, height: usize, default_value: bool) -> Self {
        let default_value = if default_value { u64::MAX } else { u64::MIN };
        let values = vec![default_value; 1 + (width * height) / 64_usize];
        BoolGrid {
            width,
            height,
            values,
        }
    }
    fn get(&self, x: usize, y: usize) -> bool {
        let ix = self.get_ix(x, y);
        (self.values[ix / 64] & (1 << (ix % 64))) != 0
    }
    fn set(&mut self, x: usize, y: usize, value: bool) {
        let ix = self.get_ix(x, y);
        if value {
            self.values[ix / 64] |= 1 << (ix % 64);
        } else {
            self.values[ix / 64] &= !(1 << (ix % 64));
        }
    }
    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.height
    }
}

impl BoolGrid {
    pub fn new(width: usize, height: usize, default_value: bool) -> BoolGrid {
        let default_value = if default_value { u64::MAX } else { u64::MIN };
        let values = vec![default_value; 1 + (width * height) / 64_usize];
        BoolGrid {
            width,
            height,
            values,
        }
    }
}

/// Generic [Grid] implementation for [Clone] and [Copy] items.
#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct SimpleGrid<T: Clone + Copy> {
    pub width: usize,
    pub height: usize,
    pub values: Vec<T>,
}

impl<T: Clone + Copy> Grid<T> for SimpleGrid<T> {
    fn new(width: usize, height: usize, default_value: T) -> Self {
        let symbols = vec![default_value; width * height];
        SimpleGrid {
            width,
            height,
            values: symbols,
        }
    }
    fn get(&self, x: usize, y: usize) -> T {
        self.values[x + y * self.width]
    }
    fn set(&mut self, x: usize, y: usize, value: T) {
        self.values[x + y * self.width] = value;
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}
