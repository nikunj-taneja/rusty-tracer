use rand::Rng;
use crate::ray::Ray;
use nalgebra::Vector3;
use std::io::{self, Write};
use crate::hittable::{Hittable, HittableList};

pub fn render_init(image_width: &i32, image_height: &i32) {
    println!("P3\n{} {}\n255", *image_width, *image_height);
}

pub fn random_unit_vector() -> Vector3<f64> {
    return random_in_unit_sphere().normalize()
}

pub fn random_in_unit_sphere() -> Vector3<f64> {
    let mut rng = rand::thread_rng();
    let unit = Vector3::new(1.0, 1.0, 1.0);
    loop {
        let p = 2.0 * Vector3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()) - unit;
        if p.magnitude_squared() < 1.0 {
            return p
        }
    }
}

pub fn color(ray: &Ray, world: &HittableList, depth: usize) -> Vector3<f64> {
    if depth <= 0 {
        return Vector3::new(0.0, 0.0, 0.0)
    }
    if let Some(hit) = world.hit(ray, 0.001, std::f64::MAX) {
        if let Some((scattered, attenuation)) = hit.material.scatter(&ray, &hit) {
            return attenuation.zip_map(&color(&scattered, &world, depth-1), |l, r| l * r)
        } else {
            return Vector3::new(0.0, 0.0, 0.0);
        }
    } else {
        let unit_direction = ray.direction.normalize();
        let t = 0.5*(unit_direction[1] + 1.0);
        (1.0 - t)*Vector3::new(1.0, 1.0, 1.0) + t*Vector3::new(0.5, 0.7, 1.0)
    }
}

pub fn reflect(v: &Vector3<f64>, n: &Vector3<f64>) -> Vector3<f64> {
    v - 2.0*v.dot(&n)*n
}