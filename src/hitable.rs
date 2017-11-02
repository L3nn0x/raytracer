use vec3::Vec3;
use ray::Ray;

#[derive(Default, Clone)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32, rec: &mut HitRecord) -> bool;
}
