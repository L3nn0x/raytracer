use aabb::{AABB, surrounding_box};
use hitable::{Hitable, HitRecord};
use ray::Ray;
use std::cmp::Ordering;

extern crate rand;

#[derive(Clone)]
pub struct BVHNode {
    left: Box<Hitable>,
    right: Box<Hitable>,
    bounding_box: AABB
}

impl BVHNode {
    pub fn new(objs: Vec<Box<Hitable>>, t0: f32, t1: f32) -> BVHNode {
        if objs.len() < 1 { panic!("The vector is empty"); }
        let mut objs = objs;
        let axis = (3.0 * rand::random::<f32>()) as i32;
        objs.sort_by(|a, b| {
            let l = a.bounding_box(0.0, 0.0).unwrap();
            let r = b.bounding_box(0.0, 0.0).unwrap();
            if l.min()[axis] - r.min()[axis] < 0.0 {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
        let left: Box<Hitable>;
        let right: Box<Hitable>;
        if objs.len() == 1 {
            left = objs[0].clone();
            right = objs[0].clone();
        } else if objs.len() == 2 {
            left = objs[0].clone();
            right = objs[1].clone();
        } else {
            left = Box::new(BVHNode::new(objs[..objs.len() / 2].to_vec(), t0, t1));
            right = Box::new(BVHNode::new(objs[objs.len() / 2..].to_vec(), t0, t1));
        }
        let box_left = left.bounding_box(t0, t1).unwrap();
        let box_right = right.bounding_box(t0, t1).unwrap();
        BVHNode{
            left: left,
            right: right,
            bounding_box: surrounding_box(box_left, box_right)
        }
    }
}

impl Hitable for BVHNode {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        if self.bounding_box.hit(&ray, tmin as f32, tmax as f32) {
            let left = self.left.hit(&ray, tmin, tmax);
            let right = self.right.hit(&ray, tmin, tmax);
            return match (left, right) {
                (Some(l), Some(r)) => if l.t < r.t { Some(l) } else { Some(r) },
                (Some(l), None) => Some(l),
                (None, Some(r)) => Some(r),
                (None, None) => None
            };
        }
        None
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(self.bounding_box.clone())
    }

    fn box_clone(&self) -> Box<Hitable> {
        Box::new((*self).clone())
    }
}
