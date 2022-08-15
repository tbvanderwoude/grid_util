//! # grid_util
//!
//! Collection of utility constructs like [Grid](grid::Grid)'s, [Point](point::Point)'s, [Rect](rect::Rect)'s, etc. Also includes some miscellaneous logic
//! like [serde] file IO helper functions.

use std::cmp::Ordering;

pub mod direction;
pub mod grid;
pub mod io;
pub mod point;
pub mod rect;

/// Takes a primary and secondary [Ordering] and evaluates it to a single [Ordering] ([Source](https://stackoverflow.com/questions/40369255/reverse-specific-key-when-sorting-with-multiple-keys)).
pub fn chain_ordering(o1: Ordering, o2: Ordering) -> Ordering {
    match o1 {
        Ordering::Equal => o2,
        _ => o1,
    }
}
