use crate::types::{Point, Direction};

pub struct Ray {
    origin: Point,
    dir: Direction,
}

impl Ray {
    pub fn new(origin: Point, direction: Direction) -> Ray {
        Ray { origin, dir: direction / direction.length() }
    }

    pub fn origin(&self) -> Point { self.origin }
    pub fn direction(&self) -> Direction { self.dir }

    pub fn at(&self, t: f32) -> Point {
        self.origin + self.dir * t
    }
}

pub struct Hit {
    t: f32,
    pos: Point,
    normal: Direction,
    facing_front: bool,
}

impl Hit {
    pub fn new(t: f32, pos: Point, normal: Direction, facing_front: bool) -> Hit {
        Hit { t, pos, normal, facing_front }
    }

    pub fn t(&self) -> f32 { self.t }
    pub fn pos(&self) -> Point { self.pos }
    pub fn normal(&self) -> Direction { self.normal }
    pub fn facing_front(&self) -> bool { self.facing_front }
}