mod image_writer;
use image_writer::write_image;

mod types;
use types::Color;

fn main() {
    let data: Vec<Color> = vec![
        [1.0, 1.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 1.0],
        [0.0, 1.0, 0.0],
        [1.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
    ];
    write_image(data, 2, 3);
}