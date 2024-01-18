use crate::linalg::{Color, Vec3};
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

				let cos_theta = hit.normal().dot(ray.direction() * -1.0).min(1.0);
				let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();
        let cannot = (index_in / index_out) * sin_theta > 1.0;

				let r0 = (1.0 - index_in / index_out) / (1.0 + index_in / index_out);
				let r0 = r0*r0;
				let result = r0 + (1.0 - r0) * (1.0 - cos_theta).powf(5.0);
        let cannot2 = result > Vec3::random_unit_vector().length();

				let direction = match cannot || cannot2 {
					true =>  ray.direction().reflect(hit.normal()),
					false => ray.direction().refract(hit.normal(), index_in, index_out)
				};

        let scattered = Ray::new(hit.pos(), direction);
        (Color::new(1.0, 1.0, 1.0), scattered, true)
    }
}
