use nalgebra::Vector3;
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::utils;

pub trait Material {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Vector3<f64>)>;
}

pub struct Lambertian {
    albedo: Vector3<f64>
}

impl Lambertian {
    pub fn new(albedo: Vector3<f64>) -> Self { Lambertian { albedo } }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vector3<f64>)> {
        let target = hit.p + hit.normal + utils::random_unit_vector();
        let scattered = Ray::new(hit.p, target - hit.p);
        Some((scattered, self.albedo))
    }
}

pub struct Metal {
    albedo: Vector3<f64>,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vector3<f64>, fuzz: f64) -> Self { Metal { albedo, fuzz } }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vector3<f64>)> {
        let reflected = utils::reflect(&ray.direction.normalize(), &hit.normal);
        let reflected_fuzzed = reflected + self.fuzz*utils::random_unit_vector();
        if reflected_fuzzed.dot(&hit.normal) > 0.0 {
            let scattered = Ray::new(hit.p, reflected_fuzzed);
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}
