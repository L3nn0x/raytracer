use hitable::{HitRecord, Hitable};
use ray::Ray;
use vec3::{Vec3, dot};
use material::Material;

use std::rc::Rc;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub mat: Rc<Material>
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, mat: Rc<Material>) -> Sphere {
        Sphere{
            center: center,
            radius: radius,
            mat: mat
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = dot(&ray.direction, &ray.direction);
        let b = dot(&oc, &ray.direction);
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let tmp = (-b - (b * b - a * c).sqrt()) / a;
            if tmp < tmax && tmp > tmin {
                let point = ray.point_at_parameter(tmp);
                return Some(HitRecord::new(tmp, point, (point - self.center) / self.radius, self.mat.clone()));
            }
            let tmp = (-b + (b * b - a * c).sqrt()) / a;
            if tmp < tmax && tmp > tmin {
                let point = ray.point_at_parameter(tmp);
                return Some(HitRecord::new(tmp, point, (point - self.center) / self.radius, self.mat.clone()));
            }
        }
        None
    }
}
