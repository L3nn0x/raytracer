use vec3::Vec3;
use ray::Ray;
use material::Material;

#[derive(Default, Clone)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub mat: &Material
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32, rec: &mut HitRecord) -> bool;
}
