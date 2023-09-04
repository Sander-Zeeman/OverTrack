use crate::scene::Scene;
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

    pub fn origin(self) -> Point { self.origin }
    pub fn direction(self) -> Direction { self.dir }

    pub fn at(self, t: f32) -> Point {
        self.origin + self.dir * t
    }

    pub fn trace(self, scene: &Scene) -> Color {
        let unit_dir = self.dir / self.dir.length();
        let a = (unit_dir.y() + 1.0) / 2.0;
        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }
}