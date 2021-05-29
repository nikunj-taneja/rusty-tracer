mod ray;
mod utils;
mod sphere;
mod camera;
mod material;
mod hittable;


use sphere::Sphere;
use camera::Camera;
use rand::prelude::*;
use rayon::prelude::*;
use nalgebra::Vector3;
use std::time::Instant;
use hittable::HittableList;
use material::{Lambertian, Metal};

fn main() {
    let start = Instant::now();

    // Image
    let aspect_ratio = 16.0/9.0;
    let image_width = 853;
    let image_height = (image_width as f64 /aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let world = HittableList::new(vec![
        Box::new(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5, Lambertian::new(Vector3::new(0.8, 0.3, 0.3)))),
        Box::new(Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0, Lambertian::new(Vector3::new(0.8, 0.8, 0.3)))),
        Box::new(Sphere::new(Vector3::new(1.0, 0.0, -1.0), 0.5, Metal::new(Vector3::new(0.8, 0.6, 0.2), 1.0))),
        Box::new(Sphere::new(Vector3::new(-1.0, 0.0, -1.0), 0.5, Metal::new(Vector3::new(0.8, 0.8, 0.8), 0.3)))
    ]);

    // Camera
    let cam = Camera::new();

    // Render 
    utils::render_init(&image_width, &image_height);
    eprintln!("Rendering image...");
    
    let img_pixels = (0..image_height).into_par_iter().rev()
        .flat_map(|j|
            (0..image_width).flat_map(|i| {
                let pixel_color: Vector3<f64> = (0..samples_per_pixel).map(|_| {
                    let mut rng = rand::thread_rng();
                    let u = (i as f64 + rng.gen::<f64>()) / (image_width-1) as f64;
                    let v = (j as f64 + rng.gen::<f64>())/ (image_height-1) as f64;
                    let ray = cam.get_ray(u, v);
                    utils::color(&ray, &world, max_depth)
                }).sum();
                pixel_color.iter().map(|c|
                    (255.99 * (c / samples_per_pixel as f64).sqrt().max(0.0).min(1.0)) as u8
                ).collect::<Vec<u8>>()
            }).collect::<Vec<u8>>()
        ).collect::<Vec<u8>>();
    
    for pixel in img_pixels.chunks(3) {
        println!("{} {} {}", pixel[0], pixel[1], pixel[2]);
    }

    eprintln!("Done! Time elapsed: {}s.", start.elapsed().as_secs());
}
