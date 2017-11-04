use vec3::Vec3;
use ray::Ray;
use hitable::HitRecord;

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
