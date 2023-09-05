use std::fs::{File, create_dir_all};
use std::io::Write;

use crate::ray::Ray;
use crate::linalg::{Point, Color, Vec3};
use crate::scene::Scene;

// Save movable camera for later
pub struct Camera {
    focal_length: f32,
    vp_width: f32,
    vp_height: f32,
    img_width: u32,
    img_height: u32
}

impl Camera {
    pub fn default() -> Camera {
        Camera {
            focal_length: 1.0,
            vp_width: 2.0 * 16.0 / 9.0,
            vp_height: 2.0,
            img_width: (400.0 * 16.0 / 9.0) as u32,
            img_height: 400
        }
    }

    pub fn new(focal_length: f32, viewport_width: f32, viewport_height: f32, img_width: u32, img_height: u32) -> Camera {
        Camera {
            focal_length,
            vp_width: viewport_width,
            vp_height: viewport_height,
            img_width,
            img_height
        }
    }

    fn write_image(data: Vec<Color>, width: u32, height: u32) {
        create_dir_all("images").expect("Failed to create directory named: images.");
        let mut file = File::create("images/test.ppm").expect("Failed to create an image file.");
    
        file.write("P3\n\n".as_bytes()).expect("Failed to write the magic number.");
        file.write(format!("{} {}\n\n", width, height).to_string().as_bytes()).expect("Failed to write width and/or height.");
        file.write("255\n\n".as_bytes()).expect("Failed to write the maximum value.");
    
        for pixel in data {
            let r = (pixel.r() * 256.0).floor().clamp(0.0, 255.0) as u8;
            let g = (pixel.g() * 256.0).floor().clamp(0.0, 255.0) as u8;
            let b = (pixel.b() * 256.0).floor().clamp(0.0, 255.0) as u8;
            file.write(format!("{} {} {}\n", r, g, b).to_string().as_bytes()).expect("Failed to write an element of data.");
        }
    }

    pub fn render(self, scene: &Scene) {
        let cam_pos = Point::default();
        let hori_step = self.vp_width / self.img_width as f32;
        let vert_step = self.vp_height / self.img_height as f32;
        let vp_top_left = cam_pos - Point::new(self.vp_width, self.vp_height, 2.0 * self.focal_length) / 2.0;
        let pixel00 = vp_top_left + Vec3::new(hori_step, vert_step, 0.0) / 2.0;

        println!("Started the render!");

        let mut data: Vec<Color> = vec![];
        for i in 0..self.img_height {
            println!("{}/{} rows remaining!", self.img_height - i, self.img_height);
            for j in 0..self.img_width {
                let pos = pixel00 + Vec3::new(j as f32 * hori_step, (self.img_height - i - 1) as f32 * vert_step, 0.0);
                let dir = pos - cam_pos;
                let ray = Ray::new(cam_pos, dir);
                data.push(scene.trace(ray));
            }
        }

        Camera::write_image(data, self.img_width, self.img_height);
        println!("Finished the render!");
    }
}