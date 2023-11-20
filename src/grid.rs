//! [Grid] trait abstracting over grid-like containers along with two implementors [BoolGrid] and [SimpleGrid].
use crate::point::Point;
use crate::rect::Rect;
use serde::{Deserialize, Serialize};

/// The [ValueGrid] trait abstracts over containers of [Clone] and [Copy] items laid out in a rectangle
/// with a certain [width](Self::width) and [height](Self::height).
pub trait ValueGrid<T: Clone + Copy> {
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
    /// Gets the index corresponding to a coordinate, which is row-wise.
    fn get_ix(&self, x: usize, y: usize) -> usize {
        x + y * self.width()
    }
    fn get_ix_point(&self, point: &Point) -> usize {
        self.get_ix(point.x as usize, point.y as usize)
    }
    /// Tests whether a point is in bounds.
    fn point_in_bounds(&self, point: Point) -> bool {
        point.x >= 0
            && point.y >= 0
            && point.x < self.width() as i32
            && point.y < self.height() as i32
    }
    /// Tests whether an index is in bounds.
    fn index_in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width() && y < self.height()
    }
    /// Sets a given rectangle on the grid to the value.
    fn set_rectangle(&mut self, rect: &Rect, value: T) {
        for x in rect.x1..rect.x2 {
            for y in rect.y1..rect.y2 {
                self.set(x as usize, y as usize, value);
            }
        }
    }
    /// Retrieves the rectangle corresponding to the grid dimensions at the origin.
    fn rect(&self) -> Rect {
        Rect::new(0, 0, self.width() as i32, self.height() as i32)
    }
    /// Retrieves a column-wise vector of grid values in the given rectangle.
    fn get_rect(&self, rect: Rect) -> Vec<T> {
        rect.points_in()
            .into_iter()
            .map(|p| self.get_point(p))
            .collect::<Vec<T>>()
    }
}

/// The [Grid] trait abstracts over containers of items laid out in a rectangle
/// with a certain [width](Self::width) and [height](Self::height). Elements are
/// accessed by reference using [get](Self::get), [get_mut](Self::get_mut) and
/// related functions.
pub trait Grid<T: Clone> {
    // Static method signature; `Self` refers to the implementor type.
    fn new(width: usize, height: usize, default_value: T) -> Self;
    fn get(&self, x: usize, y: usize) -> Option<&T>;
    fn get_point(&self, point: Point) -> Option<&T> {
        self.get(point.x as usize, point.y as usize)
    }
    fn get_ix(&self, ix: usize) -> Option<&T> {
        let w = self.width();
        let i = ix % w;
        let j = ix / w;
        self.get(i, j)
    }
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T>;
    fn get_point_mut(&mut self, point: Point) -> Option<&mut T> {
        self.get_mut(point.x as usize, point.y as usize)
    }
    fn get_ix_mut(&mut self, ix: usize) -> Option<&mut T> {
        let w = self.width();
        let i = ix % w;
        let j = ix / w;
        self.get_mut(i, j)
    }
    fn set(&mut self, x: usize, y: usize, value: T) {
        self.get_mut(x, y).map(|x| *x = value);
    }
    fn set_point(&mut self, point: Point, value: T) {
        self.set(point.x as usize, point.y as usize, value);
    }
    fn set_ix(&mut self, ix: usize, value: T) {
        let w = self.width();
        let i = ix % w;
        let j = ix / w;
        self.set(i,j, value);
    }
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    /// Gets the index corresponding to a coordinate, which is row-wise.
    fn compute_ix(&self, x: usize, y: usize) -> usize {
        x + y * self.width()
    }
    fn get_ix_point(&self, point: &Point) -> usize {
        self.compute_ix(point.x as usize, point.y as usize)
    }
    /// Tests whether a point is in bounds.
    fn point_in_bounds(&self, point: Point) -> bool {
        point.x >= 0
            && point.y >= 0
            && point.x < self.width() as i32
            && point.y < self.height() as i32
    }
    /// Tests whether an index is in bounds.
    fn index_in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width() && y < self.height()
    }
    /// Sets a given rectangle on the grid to the value.
    fn set_rectangle(&mut self, rect: &Rect, value: T) {
        for p in rect.points_in() {
            if let Some(r) = self.get_point_mut(p) {
                *r = value.clone();
            }
        }
    }
    /// Retrieves the rectangle corresponding to the grid dimensions at the origin.
    fn rect(&self) -> Rect {
        Rect::new(0, 0, self.width() as i32, self.height() as i32)
    }
    /// Retrieves a column-wise vector of grid values in the given rectangle.
    fn get_rect(&self, rect: Rect) -> Vec<T> {
        rect.points_in()
            .into_iter()
            .map(|p| self.get_point(p))
            .filter_map(|x| x.cloned())
            .collect::<Vec<T>>()
    }
}

/// Generic [Grid] implementation for [Clone] items.
#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct SimpleGrid<T: Clone> {
    pub width: usize,
    pub height: usize,
    pub values: Vec<T>,
}

impl<T: Clone> Grid<T> for SimpleGrid<T> {
    fn new(width: usize, height: usize, default_value: T) -> Self {
        let symbols = vec![default_value; width * height];
        SimpleGrid {
            width,
            height,
            values: symbols,
        }
    }
    fn get(&self, x: usize, y: usize) -> Option<&T> {
        if self.index_in_bounds(x, y) {
            Some(&self.values[x + y * self.width])
        }
        else{
            None
        }
    }
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if self.index_in_bounds(x, y) {
            Some(&mut self.values[x + y * self.width])
        }
        else{
            None
        }
    }
    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.height
    }
}

/// Compact bitwise implementation of a [ValGrid] of [bool]'s.
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct BoolGrid {
    pub width: usize,
    pub height: usize,
    pub values: Vec<u64>,
}

impl ValueGrid<bool> for BoolGrid {
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

/// Generic [ValueGrid] implementation for [Clone] and [Copy] items.
#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct SimpleValueGrid<T: Clone + Copy> {
    pub width: usize,
    pub height: usize,
    pub values: Vec<T>,
}

impl<T: Clone + Copy> ValueGrid<T> for SimpleValueGrid<T> {
    fn new(width: usize, height: usize, default_value: T) -> Self {
        let symbols = vec![default_value; width * height];
        SimpleValueGrid {
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
