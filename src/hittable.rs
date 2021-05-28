use nalgebra::Vector3;
use crate::ray::Ray;

pub struct HitRecord {
    pub p: Vector3<f64>,
    pub t: f64,
    pub normal: Vector3<f64>,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(ray: &Ray, p: Vector3<f64>, t: f64, outward_normal: Vector3<f64>) -> Self {
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        let normal = if front_face { outward_normal } else { -outward_normal };
        HitRecord { p, t, normal, front_face }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HittableList {
    list: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new(list: Vec<Box<dyn Hittable>>) -> Self { HittableList { list } }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_anything: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for obj in self.list.iter() {
            if let Some(hit) = obj.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                hit_anything = Some(hit);
            }
        }
        hit_anything
    }
}
