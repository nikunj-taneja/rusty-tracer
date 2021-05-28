mod ray;
mod sphere;
mod hittable;

use std::io::{self, Write};
use std::time::Instant;
use nalgebra::Vector3;
use ray::Ray;
use hittable::{Hittable, HittableList};
use sphere::Sphere;

pub fn color(ray: &Ray, world: &HittableList) -> Vector3<f64> {
    if let Some(hit) = world.hit(ray, 0.0, std::f64::MAX) {
        0.5*hit.normal.add_scalar(1.0)
    } else {
        let unit_direction = ray.direction.normalize();
        let t = 0.5*(unit_direction[1] + 1.0);
        (1.0 - t)*Vector3::new(1.0, 1.0, 1.0) + t*Vector3::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    let start = Instant::now();
    // Image
    let aspect_ratio = 16.0/9.0;
    let image_width = 1280;
    let image_height = (image_width as f64 /aspect_ratio) as i32;

    // World
    let world = HittableList::new(vec![
        Box::new(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.35)),
        Box::new(Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0))
    ]);

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vector3::new(0.0, 0.0, 0.0);
    let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
    let vertical = Vector3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vector3::new(0.0, 0.0, focal_length);
    
    

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rRendering image: {}%", ((image_height-j) as f64 / (image_height as f64) * 100.0) as i64);
        io::stderr().flush().unwrap();
        for i in 0..image_width {
            let u = i as f64 / (image_width-1) as f64;
            let v = j as f64 / (image_height-1) as f64;
            let r = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical - origin);
            let col = color(&r, &world);
            let ir = (255.99 * col[0]) as i32;
            let ig = (255.99 * col[1]) as i32;
            let ib = (255.99 * col[2]) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
    eprintln!("\nFinished! Time elapsed: {}s.", start.elapsed().as_secs());
}
