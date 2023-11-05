use crate::linalg::Color;
use crate::ray::{Hit, Ray};

mod dielectric;
mod lambertian;
mod metal;

pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use metal::Metal;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> (Color, Ray, bool);
}