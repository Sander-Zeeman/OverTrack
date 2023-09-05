mod camera;
mod image_writer;
mod objects;
mod ray;
mod scene;
mod types;

use camera::Camera;
use objects::sphere::Sphere;
use scene::Scene;
use types::Point;

fn main() {
    let img_width = 400;
    let img_height = (400.0  * 9.0 / 16.0) as u32;

    let camera = Camera::default();

    let mut scene = Scene::default();
    scene.add_object(Box::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    scene.add_object(Box::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));
 
    camera.render(&scene, img_width, img_height);
}