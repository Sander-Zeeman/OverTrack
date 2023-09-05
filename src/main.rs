mod camera;
mod objects;
mod ray;
mod scene;
mod linalg;

use camera::Camera;
use objects::sphere::Sphere;
use scene::Scene;
use linalg::Point;

fn main() {
    let camera = Camera::default();

    let mut scene = Scene::default();
    scene.add_object(Box::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    scene.add_object(Box::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));
 
    camera.render(&scene);
}