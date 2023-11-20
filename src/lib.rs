//! # grid_util
//!
//! Collection of utility constructs like [Grid](grid::Grid)'s, [Point](point::Point)'s, [Rect](rect::Rect)'s, etc.
//! All are geared towards a 2D grid with integer coordinates.

pub mod direction;
pub mod grid;
pub mod point;
pub mod rect;

pub use direction::Direction;
pub use grid::BoolGrid;
pub use grid::Grid;
pub use grid::SimpleGrid;
pub use grid::SimpleValueGrid;
pub use grid::ValueGrid;
pub use point::Point;
pub use rect::Rect;
