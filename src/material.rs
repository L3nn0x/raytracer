use vec3::Vec3;
use ray::Ray;
use hitable::HitRecord;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}
