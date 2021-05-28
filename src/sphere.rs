use nalgebra::Vector3;
use crate::ray::Ray;
use crate::hittable::{Hittable, HitRecord};

pub struct Sphere {
    center: Vector3<f64>,
    radius: f64
}

impl Sphere {
    pub fn new(center: Vector3<f64>, radius: f64) -> Self { Sphere { center, radius } }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let half_b = oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 { return None; }
        let sqrtd = discriminant.sqrt();
        let root = (-half_b - sqrtd)/a;
        if root < t_min || root > t_max { 
            let root = (-half_b + sqrtd)/a;
            if root < t_min || root > t_max {
                return None;
            }
        }
        let p = ray.point_at_parameter(root);
        let outward_normal: Vector3<f64> = (p - self.center)/self.radius;
        Some(HitRecord::new(ray, p, root, outward_normal))
    }
}