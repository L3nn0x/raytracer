use vec3::{Vec3, dot};
use ray::Ray;
use hitable::HitRecord;

extern crate rand;

pub struct MaterialResult {
    pub attenuation: Vec3,
    pub scattered: Ray
}

impl MaterialResult {
    pub fn new(attenuation: Vec3, scattered: Ray) -> MaterialResult {
        MaterialResult{
            attenuation: attenuation,
            scattered: scattered
        }
    }
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<MaterialResult>;
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = 2.0 * Vec3::new(rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>()) - Vec3::new(1.0, 1.0, 1.0);
        if dot(&p, &p) >= 1.0 {
            return p;
        }
    }
}
