use vec3::{Vec3, unit_vector, dot};
use ray::Ray;
use hitable::HitRecord;
use material::{Material, MaterialResult, reflect};

extern crate rand;

pub struct Dielectric {
    ref_idx: f64
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Dielectric {
        Dielectric{
            ref_idx: ref_idx
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<MaterialResult> {
        let reflected = reflect(&ray_in.direction, &rec.normal);
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let cosine = dot(&ray_in.direction, &rec.normal) / ray_in.direction.length();
        let (outward_normal, ni_over_nt, cosine) = if dot(&ray_in.direction, &rec.normal) > 0.0 {
            (-rec.normal, self.ref_idx, self.ref_idx * cosine)
        } else {
            (rec.normal, self.ref_idx.recip(), -cosine)
        };
        if let Some(refracted) = refract(&ray_in.direction, &outward_normal, ni_over_nt) {
            let reflect_prob = schlick(cosine, self.ref_idx);
            if rand::random::<f64>() < reflect_prob {
                Some(MaterialResult::new(attenuation, Ray::new(rec.p, reflected)))
            } else {
                Some(MaterialResult::new(attenuation, Ray::new(rec.p, refracted)))
            }
        } else {
            Some(MaterialResult::new(attenuation, Ray::new(rec.p, reflected)))
        }
    }
}

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let uv = unit_vector(v);
    let dt = dot(&uv, &n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some(ni_over_nt * (uv - *n * dt) - *n * discriminant.sqrt())
    } else {
        None
    }
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
