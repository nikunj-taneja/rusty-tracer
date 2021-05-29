mod ray;
mod utils;
mod sphere;
mod camera;
mod material;
mod hittable;


use sphere::Sphere;
use camera::Camera;
use rand::prelude::*;
use nalgebra::Vector3;
use std::time::Instant;
use hittable::HittableList;
use material::{Lambertian, Metal};

fn main() {
    let start = Instant::now();
    
    // Random numbers
    let mut rng = rand::thread_rng();

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
    for j in (0..image_height).rev() {
        utils::print_progress_bar(&j, &image_height);
        for i in 0..image_width {
            let mut pixel_color = Vector3::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (image_width-1) as f64;
                let v = (j as f64 + rng.gen::<f64>())/ (image_height-1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += utils::color(&r, &world, max_depth);
            }
            utils::write_color(pixel_color, samples_per_pixel);
        }
    }
    eprintln!("\nFinished! Time elapsed: {}s.", start.elapsed().as_secs());
}
