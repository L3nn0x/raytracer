use material::{Material, MaterialResult, random_in_unit_sphere};
use vec3::Vec3;
use ray::Ray;
use hitable::HitRecord;
use texture::Texture;

extern crate rand;

pub struct Lambertian {
    albedo: Box<Texture> 
}

impl Lambertian {
    pub fn new(albedo: Box<Texture>) -> Lambertian {
        Lambertian{
            albedo: albedo
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<MaterialResult> {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        Some(MaterialResult::new(self.albedo.value(0.0, 0.0, rec.p), Ray::new(rec.p, target - rec.p, ray_in.time)))
    }
}

