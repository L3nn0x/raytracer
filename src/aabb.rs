use vec3::Vec3;
use ray::Ray;

#[derive(Clone)]
pub struct AABB {
    min: Vec3,
    max: Vec3
}

impl AABB {
    pub fn new(a: Vec3, b: Vec3) -> AABB {
        AABB{
            min: a,
            max: b
        }
    }

    pub fn min(&self) -> &Vec3 {
        &self.min
    }

    pub fn max(&self) -> &Vec3 {
        &self.max
    }

    pub fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / ray.direction[a];
            let mut t0 = (self.min[a] - ray.origin[a]) * inv_d;
            let mut t1 = (self.max[a] - ray.origin[a]) * inv_d;
            if inv_d < 0.0 {
                ::std::mem::swap(&mut t0, &mut t1);
            }
            let tmin = if t0 > tmin as f64 {
                t0
            } else {
                tmin as f64
            };
            let tmax = if t1 < tmax as f64 {
                t1
            } else {
                tmax as f64
            };
            if tmax <= tmin {
                return false;
            }
        }
        true
    }
}

pub fn surrounding_box(b0: AABB, b1: AABB) -> AABB {
    let small = Vec3::new((b0.min().x).min(b1.min().x),
                (b0.min().y).min(b1.min().y),
                (b0.min().z).min(b1.min().z));
    let big = Vec3::new((b0.max().x).max(b1.max().x),
                (b0.max().y).max(b1.max().y),
                (b0.max().z).max(b1.max().z));
    AABB::new(small, big)
}
