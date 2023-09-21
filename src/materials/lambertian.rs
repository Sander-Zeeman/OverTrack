use crate::linalg::{Color, Vec3};
use crate::ray::{Ray, Hit};

use super::Material;


pub struct Lambertian {
    albedo: Color
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &Hit) -> (Color, Ray) {
        let mut direction = hit.normal() + Vec3::random_unit_vector();
        if direction.is_zero() {
            direction = hit.normal();
        }

        (self.albedo, Ray::new(hit.pos(), direction))
    }
}