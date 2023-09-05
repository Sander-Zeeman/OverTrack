use crate::image_writer::write_image;
use crate::ray::Ray;
use crate::types::{Point, Color, Vec3};
use crate::scene::Scene;

// Save movable camera for later
pub struct Camera {
    focal_length: f32,
    vp_width: f32,
    vp_height: f32,
}

impl Camera {
    pub fn default() -> Camera {
        Camera {
            focal_length: 1.0,
            vp_width: 2.0 * 16.0 / 9.0,
            vp_height: 2.0
        }
    }

    pub fn new(focal_length: f32, viewport_width: f32, viewport_height: f32) -> Camera {
        Camera {
            focal_length,
            vp_width: viewport_width,
            vp_height: viewport_height
        }
    }

    pub fn render(self, scene: &Scene, img_width: u32, img_height: u32) {
        let cam_pos = Point::default();
        let hori_step = self.vp_width / img_width as f32;
        let vert_step = self.vp_height / img_height as f32;
        let vp_top_left = cam_pos - Point::new(self.vp_width, self.vp_height, 2.0 * self.focal_length) / 2.0;
        let pixel00 = vp_top_left + Vec3::new(hori_step, vert_step, 0.0) / 2.0;

        println!("Started the render!");

        let mut data: Vec<Color> = vec![];
        for i in 0..img_height {
            println!("{}/{} rows remaining!", img_height - i, img_height);
            for j in 0..img_width {
                let pos = pixel00 + Vec3::new(j as f32 * hori_step, (img_height - i - 1) as f32 * vert_step, 0.0);
                let dir = pos - cam_pos;
                let ray = Ray::new(cam_pos, dir);
                data.push(scene.trace(ray));
            }
        }

        write_image(data, img_width, img_height);
        println!("Finished the render!");
    }
}