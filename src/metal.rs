use material::{Material, MaterialResult, random_in_unit_sphere};
use vec3::{Vec3, dot};
use ray::Ray;
use hitable::HitRecord;

#[derive(Clone)]
pub struct Metal {
    albedo: Vec3,
    fuzzy: f32
}

impl Metal {
    pub fn new(albedo: Vec3, f: f32) -> Metal {
        let f = if f > 1.0 {
            1.0
        } else {
            f
        };
        Metal{
            albedo: albedo,
            fuzzy: f
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<MaterialResult> {
        let reflected = reflect(ray_in.direction(), &rec.normal);
        let reflected = reflected + self.fuzzy * random_in_unit_sphere();
        let res = MaterialResult::new(self.albedo.clone(), Ray::new(rec.p, reflected));
        if dot(&res.scattered.direction(), &rec.normal) > 0.0 {
            return Some(res);
        }
        None
    }
}

fn reflect(v: Vec3, n: &Vec3) -> Vec3 {
    v - 2.0 * dot(&v, &n) * *n
}
