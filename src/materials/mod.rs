use crate::linalg::Color;
use crate::ray::{Hit, Ray};

pub mod lambertian;
pub mod metal;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> (Color, Ray);
}