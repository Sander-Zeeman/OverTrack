mod camera;
mod materials;
mod objects;
mod ray;
mod scene;
mod linalg;

use std::rc::Rc;

use camera::Camera;
use materials::{Lambertian, Metal, Dielectric};
use objects::sphere::Sphere;
use scene::Scene;
use linalg::{Color, Point};

fn main() {
    let camera = Camera::new(
        1.0,
        2.0 * 16.0 / 9.0,
        2.0,
        (400.0 * 16.0 / 9.0) as u32,
        400,
        100,
        50
    );

    let ground_material = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let center_material = Rc::new(Dielectric::new(1.5));
    let left_material = Rc::new(Dielectric::new(1.5));
    let right_material = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    let ground_obj = Box::new(Sphere::new(
        Point::new(0.0, -100.5, -1.0),
        100.0,
        ground_material
    ));
    let center_obj = Box::new(Sphere::new(
        Point::new(0.0, 0.0, -1.0),
        0.5,
        center_material
    ));
    let left_obj = Box::new(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        0.5,
        left_material
    ));
    let right_obj = Box::new(Sphere::new(
        Point::new(1.0, 0.0, -1.0),
        0.5,
        right_material
    ));

    let mut scene = Scene::default();
    scene.add_object(ground_obj);
    scene.add_object(center_obj);
    scene.add_object(left_obj);
    scene.add_object(right_obj);

    camera.render(&scene);
}