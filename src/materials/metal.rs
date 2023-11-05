use crate::linalg::{Color, Vec3};
use crate::ray::{Ray, Hit};

use super::Material;


pub struct Metal {
    albedo: Color,
    fuzziness: f32
}

impl Metal {
    pub fn new(albedo: Color, f: f32) -> Metal {
        Metal { albedo, fuzziness: f.clamp(0.0, 1.0) }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> (Color, Ray, bool) {
        let reflected = ray.direction().reflect(hit.normal());
        let scattered = Ray::new(hit.pos(), reflected + Vec3::random_unit_vector() * self.fuzziness);
        let proceed = scattered.direction().dot(hit.normal()) > 0.0;
        (self.albedo, scattered, proceed)
    }
}