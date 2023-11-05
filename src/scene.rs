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
            return Color::new(1.0, 1.0, 1.0);
        }

        let mut closest_hit: Option<Hit> = None;

        for obj in &self.objects {
            match obj.intersects(&ray) {
                Some(hit) => {
                    if hit.t() < 0.001 {
                        continue;
                    }

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
                let (attenuation, scattered, proceed) = hit.material().scatter(&ray, &hit);
                if !proceed {
                    return Color::new(0.0, 0.0, 0.0);
                }

                attenuation * self.trace(scattered, depth - 1)
            },
            None => {
                let a = (ray.direction().y() + 1.0) / 2.0;
                Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
            }
        }
    }
}