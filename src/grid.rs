//! [Grid] trait abstracting over grid-like containers along with two implementors [BoolGrid] and [SimpleGrid].
use crate::point::Point;
use crate::rect::Rect;
use serde::{Deserialize, Serialize};

/// The [ValueGrid] trait abstracts over containers of [Clone] and [Copy] items laid out in a rectangle
/// with a certain [width](Self::width) and [height](Self::height).
pub trait ValueGrid<T: Clone + Copy> {
    // Static method signature; `Self` refers to the implementor type.
    fn new(width: usize, height: usize, default_value: T) -> Self;
    fn get(&self, x: i32, y: i32) -> T;
    fn get_point(&self, point: Point) -> T {
        self.get(point.x, point.y)
    }
    fn set_point(&mut self, point: Point, value: T) {
        self.set(point.x, point.y, value);
    }
    fn set(&mut self, x: i32, y: i32, value: T);
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    /// Gets the index corresponding to a coordinate, which is row-wise.
    fn get_ix(&self, x: i32, y: i32) -> usize {
        x as usize + (y as usize) * self.width()
    }
    fn get_ix_point(&self, point: &Point) -> usize {
        self.get_ix(point.x, point.y)
    }
    /// Tests whether a point is in bounds.
    fn point_in_bounds(&self, point: Point) -> bool {
        self.index_in_bounds(point.x, point.y)
    }
    /// Tests whether an index is in bounds.
    fn index_in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.width() as i32 && y < self.height() as i32
    }
    /// Sets a given rectangle on the grid to the value.
    fn set_rect(&mut self, rect: Rect, value: T) {
        for x in rect.x1..rect.x2 {
            for y in rect.y1..rect.y2 {
                self.set(x, y, value);
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
pub trait Grid<T> {
    // Static method signature; `Self` refers to the implementor type.
    fn new(width: usize, height: usize, default_value: T) -> Self
    where
        T: Clone;
    fn get(&self, x: i32, y: i32) -> Option<&T>;
    fn get_point(&self, point: Point) -> Option<&T> {
        self.get(point.x, point.y)
    }
    fn get_ix(&self, ix: usize) -> Option<&T> {
        let w = self.width();
        let i = ix % w;
        let j = ix / w;
        self.get(i as i32, j as i32)
    }
    fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut T>;
    fn get_point_mut(&mut self, point: Point) -> Option<&mut T> {
        self.get_mut(point.x, point.y)
    }
    fn get_ix_mut(&mut self, ix: usize) -> Option<&mut T> {
        let w = self.width();
        let i = ix % w;
        let j = ix / w;
        self.get_mut(i as i32, j as i32)
    }
    fn set(&mut self, x: i32, y: i32, value: T) {
        self.get_mut(x, y).map(|x| *x = value);
    }
    fn set_point(&mut self, point: Point, value: T) {
        self.set(point.x, point.y, value);
    }
    fn set_ix(&mut self, ix: usize, value: T) {
        let w = self.width();
        let i = ix % w;
        let j = ix / w;
        self.set(i as i32, j as i32, value);
    }
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    /// Gets the index corresponding to a coordinate, which is row-wise.
    fn compute_ix(&self, x: i32, y: i32) -> usize {
        x as usize + y as usize * self.width()
    }
    fn get_ix_point(&self, point: &Point) -> usize {
        self.compute_ix(point.x, point.y)
    }
    /// Tests whether a point is in bounds.
    fn point_in_bounds(&self, point: Point) -> bool {
        self.index_in_bounds(point.x, point.y)
    }
    /// Tests whether an index is in bounds.
    fn index_in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.width() as i32 && y < self.height() as i32
    }
    /// Sets a given rectangle on the grid to the value.
    fn set_rect(&mut self, rect: Rect, value: T)
    where
        T: Clone,
    {
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
    fn get_rect(&self, rect: Rect) -> Vec<T>
    where
        T: Clone,
    {
        rect.points_in()
            .into_iter()
            .map(|p| self.get_point(p))
            .filter_map(|x| x.cloned())
            .collect::<Vec<T>>()
    }
}

/// Generic [Grid] implementation.
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct SimpleGrid<T> {
    pub width: usize,
    pub height: usize,
    pub values: Vec<T>,
}

impl<T> Grid<T> for SimpleGrid<T> {
    fn new(width: usize, height: usize, default_value: T) -> Self
    where
        T: Clone,
    {
        let symbols = vec![default_value; width * height];
        SimpleGrid {
            width,
            height,
            values: symbols,
        }
    }
    fn get(&self, x: i32, y: i32) -> Option<&T> {
        if self.index_in_bounds(x, y) {
            Some(&self.values[self.compute_ix(x, y)])
        } else {
            None
        }
    }
    fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut T> {
        if self.index_in_bounds(x, y) {
            let ix = self.compute_ix(x, y);
            Some(&mut self.values[ix])
        } else {
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

impl<T: Clone> Clone for SimpleGrid<T> {
    fn clone(&self) -> Self {
        Self { width: self.width.clone(), height: self.height.clone(), values: self.values.clone() }
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
    fn get(&self, x: i32, y: i32) -> bool {
        let ix = self.get_ix(x, y);
        (self.values[ix / 64] & (1 << (ix % 64))) != 0
    }
    fn set(&mut self, x: i32, y: i32, value: bool) {
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
    fn get(&self, x: i32, y: i32) -> T {
        self.values[self.get_ix(x, y)]
    }
    fn set(&mut self, x: i32, y: i32, value: T) {
        let ix = self.get_ix(x, y);
        self.values[ix] = value;
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_out_of_bounds() {
        let grid = SimpleGrid::new(3, 2, true);
        assert_eq!(grid.get(10, 10), None);
        assert_eq!(grid.get(-10, -10), None);
        assert_eq!(grid.get(10, -10), None);
        assert_eq!(grid.get(-10, 10), None);
    }

    #[test]
    fn test_simple_grid() {
        let mut grid = SimpleGrid::new(3, 2, true);
        assert_eq!(grid.width(), 3);
        assert_eq!(grid.height(), 2);
        grid.set(1, 1, false);
        assert_eq!(*grid.get(1, 1).unwrap(), false);
    }
}
