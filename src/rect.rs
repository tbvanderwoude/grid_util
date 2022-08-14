use crate::point::Point;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub struct Rect {
    pub x1: i32,
    pub x2: i32,
    pub y1: i32,
    pub y2: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Rect {
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }
    pub fn width(&self) -> i32 {
        self.x2 - self.x1
    }
    pub fn area(&self) -> i32 {
        self.width() * self.height()
    }
    pub fn height(&self) -> i32 {
        self.y2 - self.y1
    }
    pub fn points_in(&self) -> Vec<Point> {
        let mut vec = vec![];
        for x in self.x1..=self.x2 {
            for y in self.y1..=self.y2 {
                vec.push(Point::new(x, y));
            }
        }
        vec
    }
    pub fn split_x(&self, x: i32) -> (Rect, Rect) {
        return (
            Rect::new(self.x1, self.y1, x, self.height()),
            Rect::new(
                self.x1 + x + 1,
                self.y1,
                self.width() - x - 1,
                self.height(),
            ),
        );
    }
    pub fn split_y(&self, y: i32) -> (Rect, Rect) {
        return (
            Rect::new(self.x1, self.y1, self.width(), y),
            Rect::new(
                self.x1,
                self.y1 + y + 1,
                self.width(),
                self.height() - y - 1,
            ),
        );
    }
    pub fn inner_rect(&self) -> Rect {
        Rect {
            x1: self.x1 + 1,
            x2: self.x2 - 1,
            y1: self.y1 + 1,
            y2: self.y2 - 1,
        }
    }
    pub fn intersect(&self, other: &Rect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }
    pub fn sample_point(&self) -> Point {
        Point::new(
            thread_rng().gen_range(self.x1..=self.x2),
            thread_rng().gen_range(self.y1..=self.y2),
        )
    }
    pub fn contains(&self, point: &Point) -> bool {
        self.x1 <= point.x && point.x <= self.x2 && self.y1 <= point.y && point.y <= self.y2
    }
    pub fn center(&self) -> Point {
        Point::new((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }
}
