use material::{Material, MaterialResult, random_in_unit_sphere};
use ray::Ray;
use hitable::HitRecord;
use texture::Texture;
use std::sync::Arc;

extern crate rand;

pub struct Lambertian {
    albedo: Arc<Texture> 
}

impl Lambertian {
    pub fn new(albedo: Arc<Texture>) -> Lambertian {
        Lambertian{
            albedo: albedo
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<MaterialResult> {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        Some(MaterialResult::new(self.albedo.value(rec.u as f32, rec.v as f32, rec.p), Ray::new(rec.p, target - rec.p, ray_in.time)))
    }
}

