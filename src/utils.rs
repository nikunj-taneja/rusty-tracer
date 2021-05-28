use nalgebra::Vector3;
use std::io::{self, Write};

pub fn render_init(image_width: &i32, image_height: &i32) {
    println!("P3\n{} {}\n255", *image_width, *image_height);
}

pub fn print_progress_bar(j: &i32, image_height: &i32) {
    eprint!("\rRendering image: {}%", ((*image_height-*j) as f64 / (*image_height as f64) * 100.0) as i64);
    io::stderr().flush().unwrap();
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min { min }
    else if x > max { max }
    else { x }
}

pub fn write_color(pixel_color: Vector3<f64>, samples_per_pixel: usize) {
    let scale = 1.0 / samples_per_pixel as f64;
    let ir = (256.0 * clamp(pixel_color[0]*scale, 0.0, 0.999)) as i32;
    let ig = (256.0 * clamp(pixel_color[1]*scale, 0.0, 0.999)) as i32;
    let ib = (256.0 * clamp(pixel_color[2]*scale, 0.0, 0.999)) as i32;
    println!("{} {} {}", ir, ig, ib);
}