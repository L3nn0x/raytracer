use material::Material;
use vec3::{Vec3, dot};
use ray::Ray;
use hitable::HitRecord;

extern crate rand;

#[derive(Clone)]
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
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        *scattered = Ray::new(rec.p, target - rec.p);
        *attenuation = self.albedo.clone();
        true
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = 2.0 * Vec3::new(rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>()) - Vec3::new(1.0, 1.0, 1.0);
        if dot(&p, &p) >= 1.0 {
            return p;
        }
    }
}
