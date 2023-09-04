use crate::objects::Object;
use crate::ray::Ray;
use crate::types::Point;

pub struct Circle {
    center: Point,
    radius: f32,
}

impl Object for Circle {
    fn intersects(self, ray: Ray) -> bool {
        true
    }
}