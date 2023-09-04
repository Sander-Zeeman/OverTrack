mod image_writer;
use image_writer::write_image;

mod types;
use types::{Vec3, Color};

fn main() {
    let data: Vec<Color> = vec![
        Vec3::new(1.0, 1.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 1.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(1.0, 0.0, 1.0),
        Vec3::new(0.0, 0.0, 1.0),
    ];
    write_image(data, 2, 3);
}