pub mod sphere;

use crate::ray::{Hit, Ray};

pub trait Object {
    fn intersects(&self, ray: &Ray) -> Option<Hit>;
}