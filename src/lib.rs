use std::cmp::Ordering;

pub mod direction;
pub mod grid;
pub mod io;
pub mod point;
pub mod rect;

// https://stackoverflow.com/questions/40369255/reverse-specific-key-when-sorting-with-multiple-keys
pub fn chain_ordering(o1: Ordering, o2: Ordering) -> Ordering {
    match o1 {
        Ordering::Equal => o2,
        _ => o1,
    }
}
