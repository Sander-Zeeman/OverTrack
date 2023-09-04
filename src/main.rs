mod camera;
mod image_writer;
mod objects;
mod ray;
mod scene;
mod types;

use camera::Camera;
use image_writer::write_image;
use scene::Scene;
use types::{Vec3, Color};

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let width = (400.0 * 16.0 / 9.0) as i32;
    let height = 400;

    let camera = Camera::default();
    let scene = Scene::default();
    let data = camera.render(scene, width, height);

    // write_image(data, 2, 3);
}