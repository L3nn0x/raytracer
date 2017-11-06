use vec3::Vec3;
use ray::Ray;
use material::Material;
use aabb::AABB;

use std::sync::Arc;

pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub mat: Arc<Material>,
    pub u: f64,
    pub v: f64
}

impl HitRecord {
    pub fn new(t: f64, p: Vec3, normal: Vec3, mat: Arc<Material>, u: f64, v: f64) -> HitRecord {
        HitRecord{
            t: t,
            p: p,
            normal: normal,
            mat: mat,
            u: u,
            v: v
        }
    }
}

pub trait Hitable: Sync {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB>;
    fn box_clone(&self) -> Box<Hitable>;
}

impl Clone for Box<Hitable> {
    fn clone(&self) -> Box<Hitable> {
        self.box_clone()
    }
}
