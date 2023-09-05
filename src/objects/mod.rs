pub mod sphere;

use crate::ray::{Hit, Ray};
use crate::linalg::{Direction, Point};

pub trait Object {
    fn intersects(&self, ray: &Ray) -> Option<Hit>;
    fn normal(&self, hit_point: Point) -> Direction;
}