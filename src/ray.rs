use crate::types::{Point, Direction, Color};

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

    pub fn origin(&self) -> Point { self.origin }
    pub fn direction(&self) -> Direction { self.dir }

    pub fn at(self, t: f32) -> Point {
        self.origin + self.dir * t
    }
}

pub struct Hit {
    t: f32,
    color: Color
}

impl Hit {
    pub fn new(t: f32, color: Color) -> Hit {
        Hit { t, color }
    }

    pub fn t(&self) -> f32 { self.t }
    pub fn color(&self) -> Color { self.color }
}