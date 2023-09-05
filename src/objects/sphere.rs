use crate::objects::Object;
use crate::ray::{Hit, Ray};
use crate::linalg::{Point, Direction};

pub struct Sphere {
    center: Point,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Point, radius: f32) -> Sphere {
        Sphere { center, radius }
    }
}

impl Object for Sphere {
    fn intersects(&self, ray: &Ray) -> Option<Hit> {
        let ray_sphere = ray.origin() - self.center;
        let ray_dir = ray.direction();

        let a = ray_dir.length_sq();
        let b = ray_dir.dot(ray_sphere) * 2.0;
        let c = ray_sphere.length_sq() - self.radius * self.radius;

        let disc = b*b - 4.0*a*c;

        if disc < 0.0 {
            return None;
        }

        let t1 = (-b + disc.sqrt()) / (2.0 * a);
        let t2 = (-b - disc.sqrt()) / (2.0 * a);
        let t;

        if t1 < 0.0 && t2 < 0.0 {
            return None;
        }

        if t1 >= 0.0 && t2 >= 0.0 {
            t = t1.min(t2);            
        } else {
            t = t1.max(t2);
        }

        let mut N = self.normal(ray.at(t));
        let mut facing_front = true;
        if ray_dir.dot(N) > 0.0 {
            N = N * -1.0;
            facing_front = false;
        }

        Some(Hit::new(t, ray.at(t), N, facing_front))
    }

    fn normal(&self, hit_point: Point) -> Direction {
        (hit_point - self.center) / self.radius
    }
}