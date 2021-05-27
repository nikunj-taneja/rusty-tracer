mod ray;

use std::io::{self, Write};
use std::time::Instant;
use nalgebra::Vector3;
use ray::Ray;

fn hit_sphere(center: &Vector3<f64>, radius: f64, ray: &Ray) -> Option<f64> {
    let oc = ray.origin - center;
    let a = ray.direction.dot(&ray.direction);
    let b = 2.0 * oc.dot(&ray.direction);
    let c = oc.dot(&oc) - radius.powi(2);
    let discriminant = b.powi(2) - 4.0 * a * c;
    if discriminant < 0.0 {
        None
    } else {
        Some((-b - discriminant.sqrt())/(2.0*a))
    }
}

pub fn color(ray: &Ray) -> Vector3<f64> {
    if let Some(t) = hit_sphere(&Vector3::new(0.0, 0.0, -1.0), 0.5, ray) {
        let normal = (ray.point_at_parameter(t) - Vector3::new(0.0, 0.0, -1.0)).normalize();
        0.5*normal.add_scalar(1.0)
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
        eprint!("\rScanlines remaining: {}", j);
        io::stderr().flush().unwrap();
        for i in 0..image_width {
            let u = i as f64 / (image_width-1) as f64;
            let v = j as f64 / (image_height-1) as f64;
            let r = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical - origin);
            let col = color(&r);
            let ir = (255.99 * col[0]) as i32;
            let ig = (255.99 * col[1]) as i32;
            let ib = (255.99 * col[2]) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
    eprintln!("\nFinished! Time elapsed: {}s.", start.elapsed().as_secs());
}
