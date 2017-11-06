use hitable::{HitRecord, Hitable};
use ray::Ray;
use vec3::{Vec3, dot};
use material::Material;
use aabb::{AABB, surrounding_box};

use std::sync::Arc;
use std::f64::consts::PI;

#[derive(Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub mat: Arc<Material>
}

#[derive(Clone)]
pub struct MovingSphere {
    center0: Vec3,
    center1: Vec3,
    time0: f32,
    time1: f32,
    pub radius: f64,
    pub mat: Arc<Material>
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, mat: Arc<Material>) -> Sphere {
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
                let (u, v) = get_sphere_uv((point - self.center) / self.radius);
                return Some(HitRecord::new(tmp, point, (point - self.center) / self.radius, self.mat.clone(), u, v));
            }
            let tmp = (-b + (b * b - a * c).sqrt()) / a;
            if tmp < tmax && tmp > tmin {
                let point = ray.point_at_parameter(tmp);
                let (u, v) = get_sphere_uv((point - self.center) / self.radius);
                return Some(HitRecord::new(tmp, point, (point - self.center) / self.radius, self.mat.clone(), u, v));
            }
        }
        None
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB::new(self.center - Vec3::new(self.radius, self.radius, self.radius),
                        self.center + Vec3::new(self.radius, self.radius, self.radius)))
    }

    fn box_clone(&self) -> Box<Hitable> {
        Box::new((*self).clone())
    }
}

impl MovingSphere {
    pub fn new(center0: Vec3, center1: Vec3, radius: f64, time0: f32, time1: f32,
               mat: Arc<Material>) -> MovingSphere {
        MovingSphere{
            center0: center0,
            center1: center1,
            radius: radius,
            time0: time0,
            time1: time1,
            mat: mat
        }
    }

    pub fn center(&self, time: f32) -> Vec3 {
        self.center0 + (time - self.time0) as f64 / (self.time1 - self.time0) as f64 * (self.center1 - self.center0)
    }
}

impl Hitable for MovingSphere {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center(ray.time);
        let a = dot(&ray.direction, &ray.direction);
        let b = dot(&oc, &ray.direction);
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let tmp = (-b - (b * b - a * c).sqrt()) / a;
            if tmp < tmax && tmp > tmin {
                let point = ray.point_at_parameter(tmp);
                let (u, v) = get_sphere_uv((point - self.center(ray.time)) / self.radius);
                return Some(HitRecord::new(tmp, point, (point - self.center(ray.time)) / self.radius, self.mat.clone(), u, v));
            }
            let tmp = (-b + (b * b - a * c).sqrt()) / a;
            if tmp < tmax && tmp > tmin {
                let point = ray.point_at_parameter(tmp);
                let (u, v) = get_sphere_uv((point - self.center(ray.time)) / self.radius);
                return Some(HitRecord::new(tmp, point, (point - self.center(ray.time)) / self.radius, self.mat.clone(), u, v));
            }
        }
        None
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        let b0 = AABB::new(self.center(t0) - Vec3::new(self.radius, self.radius, self.radius),
                            self.center(t0) - Vec3::new(self.radius, self.radius, self.radius));
        let b1 = AABB::new(self.center(t1) - Vec3::new(self.radius, self.radius, self.radius),
                            self.center(t1) - Vec3::new(self.radius, self.radius, self.radius));
        Some(surrounding_box(b0, b1))
    }

    fn box_clone(&self) -> Box<Hitable> {
        Box::new((*self).clone())
    }
}

fn get_sphere_uv(p: Vec3) -> (f64, f64) {
    let phi = p.z.atan2(p.x);
    let theta = p.y.asin();
    (1.0 - (phi + PI) / (2.0 * PI), (theta + PI / 2.0) / PI)
}
