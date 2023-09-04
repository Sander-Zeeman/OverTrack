pub mod sphere;

use crate::ray::Ray;

pub trait Object {
    fn intersects(self, ray: Ray) -> bool;
}