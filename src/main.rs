mod camera;
mod image_writer;
mod objects;
mod ray;
mod scene;
mod types;

use camera::Camera;
use image_writer::write_image;
use objects::sphere::Sphere;
use scene::Scene;
use types::Point;

fn main() {
    let img_width = (400.0 * 16.0 / 9.0) as u32;
    let img_height = 400;

    let camera = Camera::default();
    let mut scene = Scene::default();
    scene.add_object(Box::new(Sphere::new(Point::new(0.0, 0.0, -5.0), 2.0)));
 
    let data = camera.render(&scene, img_width, img_height);

    write_image(data, img_width, img_height);
}