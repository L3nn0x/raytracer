use material::{Material, MaterialResult, random_in_unit_sphere};
use vec3::Vec3;
use ray::Ray;
use hitable::HitRecord;

extern crate rand;

pub struct Lambertian {
    albedo: Vec3
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian{
            albedo: albedo
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord) -> Option<MaterialResult> {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        Some(MaterialResult::new(self.albedo.clone(), Ray::new(rec.p, target - rec.p)))
    }
}

