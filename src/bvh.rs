use aabb::{AABB, surrounding_box};
use hitable::{Hitable, HitRecord};
use ray::Ray;

extern crate rand;

pub struct BVHNode {
    left: Box<Hitable>,
    right: Box<Hitable>,
    bounding_box: AABB
}

impl BVHNode {
    pub fn new(objs: Vec<Box<Hitable>>, t0: f32, t1: f32) -> BVHNode {
        let mut objs = objs;
        let axis = (3.0 * rand::random::<f32>()) as i32;
        objs.sort_by(|a, b| {
            let l = a.bounding_box(0, 0).unwrap();
            let r = b.bounding_box(0, 0).unwrap();
            l.min()[axis].cmp(r.min()[axis])
        });
        let mut left: Box<Hitable>;
        let mut rght: Box<Hitable>;
        if objs.len() == 1 {
            left = objs[0].clone();
            right = objs[0].clone();
        } else if objs.len() == 2 {
            left = objs[0];
            right = objs[1];
        } else {
            left = BVHNode::new(&objs[..objs.len() / 2], t0, t1);
            right = BVHNode::new(&objs[objs.len() / 2..], t0, t1);
        }
        let box_left = left.bounding_box(t0, t1).unwrap();
        let box_right = left.bounding_box(t0, t1).unwrap();
        BVHNode{
            left: left,
            right: right,
            bounding_box: surrounding_box(box_left, box_right)
        }
    }
}

impl Hitable for BVHNode {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        if self.bounding_box.hit(tmin, tmax) {
            let left = self.left.hit(&ray, tmin, tmax);
            let right = self.right.hit(&ray, tmin, tmax);
            match (left, right) {
                (Some(l), Some(r)) => if l.t < r.t { left } else { right },
                (Some(_), None) => left,
                (None, Some(_)) => right,
                (None, None) => None
            }
        }
        None
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        Some(self.bounding_box.clone())
    }
}
