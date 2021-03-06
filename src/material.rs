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

pub trait Material: Sync + Send {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<MaterialResult>;
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = 2.0 * Vec3::new(rand::random::<f64>(), rand::random::<f64>(), rand::random::<f64>()) - Vec3::new(1.0, 1.0, 1.0);
        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2.0 * dot(&v, &n) * *n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reflect_test() {
        let a = Vec3::new(1.0, 1.0, 0.0);
        let n = Vec3::new(0.0, 1.0, 0.0);
        assert_eq!(reflect(&a, &n), Vec3::new(1.0, -1.0, 0.0));
        let a = Vec3::new(-1.0, -1.0, 0.0);
        assert_eq!(reflect(&a, &n), Vec3::new(-1.0, 1.0, 0.0));
}
}
