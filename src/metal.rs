use material::Material;
use vec3::{Vec3, dot};
use ray::Ray;
use hitable::HitRecord;

pub struct Metal {
    albedo: Vec3
}

impl Metal {
    pub fn new(albedo: Vec3) -> Metal {
        Metal{
            albedo: albedo
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected = reflect(ray_in.direction(), &rec.normal);
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo.clone();
        dot(&scattered.direction, &rec.normal) > 0.0
    }
}

fn reflect(v: Vec3, n: &Vec3) -> Vec3 {
    v - 2.0 * dot(&v, &n) * *n
}
