use crate::objects::Object;
use crate::ray::{Hit, Ray};
use crate::types::Color;

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

    pub fn trace(&self, ray: Ray) -> Color {
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
            Some(hit) => hit.color(),
            None => Color::default()
        }
    }
}