use crate::types::{Point, Direction};

pub struct Ray {
    origin: Point,
    dir: Direction,
}

impl Ray {
    pub fn default() -> Ray {
        Ray { origin: Point::default(), dir: Direction::default() }
    }

    pub fn new(origin: Point, direction: Direction) -> Ray {
        Ray { origin, dir: direction }
    }

    pub fn origin(self) -> Point { self.origin }
    pub fn direction(self) -> Direction { self.dir }

    pub fn at(self, t: f32) -> Point {
        self.origin + self.dir * t
    }
}