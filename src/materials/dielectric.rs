use crate::linalg::Color;
use crate::ray::{Ray, Hit};

use super::Material;

const AIR_INDEX: f32 = 1.0;

pub struct Dielectric {
    refraction_index: f32
}

impl Dielectric {
    pub fn new(refraction_index: f32) -> Dielectric {
        Dielectric { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> (Color, Ray, bool) {
        let (index_in, index_out) = match hit.facing_front() {
            true => (AIR_INDEX, self.refraction_index),
            false => (self.refraction_index, AIR_INDEX)
        };
        let refracted = ray.direction().refract(hit.normal(), index_in, index_out);
        let scattered = Ray::new(hit.pos(), refracted);
        (Color::new(1.0, 1.0, 1.0), scattered, true)
    }
}