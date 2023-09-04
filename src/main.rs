mod camera;
mod image_writer;
mod objects;
mod ray;
mod scene;
mod types;

use camera::Camera;
use image_writer::write_image;
use scene::Scene;

fn main() {
    let img_width = (400.0 * 16.0 / 9.0) as u32;
    let img_height = 400;

    let camera = Camera::default();
    let scene = Scene::default();
 
    let data = camera.render(&scene, img_width, img_height);

    write_image(data, img_width, img_height);
}