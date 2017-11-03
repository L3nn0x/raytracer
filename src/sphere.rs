use hitable::{HitRecord, Hitable};
use ray::Ray;
use vec3::{Vec3, dot};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere{
            center: center,
            radius: radius
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32, rec: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let a = dot(&ray.direction, &ray.direction);
        let b = dot(&oc, &ray.direction);
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let tmp = (-b - (b * b - a * c).sqrt()) / a;
            if tmp < tmax && tmp > tmin {
                rec.t = tmp;
                rec.p = ray.point_at_parameter(tmp);
                rec.normal = (rec.p - self.center) / self.radius;
                return true;
            }
            let tmp = (-b + (b * b - a * c).sqrt()) / a;
            if tmp < tmax && tmp > tmin {
                rec.t = tmp;
                rec.p = ray.point_at_parameter(tmp);
                rec.normal = (rec.p - self.center) / self.radius;
                return true;
            }
        }
        false
    }
}
