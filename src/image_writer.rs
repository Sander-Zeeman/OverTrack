// Writes images in the PPM format
use std::fs::{File, create_dir_all};
use std::io::Write;

use crate::types::Color;

pub fn write_image(data: Vec<Color>, width: u32, height: u32) {
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
