use crate::linalg::Color;
use crate::ray::{Ray, Hit};

use super::Material;


pub struct Metal {
    albedo: Color
}

impl Metal {
    pub fn new(albedo: Color) -> Metal {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> (Color, Ray) {
        let reflected = ray.direction().reflect(hit.normal());
        (self.albedo, Ray::new(hit.pos(), reflected))
    }
}