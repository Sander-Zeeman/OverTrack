use crate::types::{Point, Direction, Color};
use crate::scene::Scene;

pub struct Camera {
    position: Point,
    direction: Direction,

    focal_length: f32,
    vp_width: f32,
    vp_height: f32,
}

impl Camera {
    pub fn default() -> Camera {
        Camera {
            position: Point::default(),
            direction: Direction::default(),
            focal_length: 1.0,
            vp_width: 2.0 * 16.0 / 9.0,
            vp_height: 2.0
        }
    }

    pub fn new(origin: Point, direction: Direction, focal_length: f32, viewport_width: f32, viewport_height: f32) -> Camera {
        Camera {
            position: origin,
            direction,
            focal_length,
            vp_width: viewport_width,
            vp_height: viewport_height
        }
    }

    pub fn render(scene: Scene, img_width: f32, img_height: f32) -> Vec<Color> {
        vec![]
    }
}