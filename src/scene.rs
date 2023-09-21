use crate::objects::Object;
use crate::ray::{Hit, Ray};
use crate::linalg::Color;

pub struct Scene {
    objects: Vec<Box<dyn Object>>
}

impl Scene {
    pub fn default() -> Scene {
        Scene { objects: vec![] }
    }

    pub fn add_object(&mut self, obj: Box<dyn Object>) {
        self.objects.push(obj);
    }

    pub fn trace(&self, ray: Ray, depth: u32) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut closest_hit: Option<Hit> = None;

        for obj in &self.objects {
            match obj.intersects(&ray) {
                Some(hit) => {
                    match &closest_hit {
                        Some(closest) => {
                            if hit.t() < closest.t() {
                                closest_hit = Some(hit);
                            }
                        },
                        None => closest_hit = Some(hit)
                    }
                }
                None => continue
            }
        }

        match closest_hit {
            Some(hit) => {
                let (attenuation, scattered) = hit.material().scatter(&ray, &hit);
                let acneless_scattered = Ray::new(scattered.origin() + hit.normal() * 0.001, scattered.direction());
                attenuation * self.trace(acneless_scattered, depth - 1)
            },
            None => {
                let a = (ray.direction().y() + 1.0) / 2.0;
                Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
            }
        }
    }
}