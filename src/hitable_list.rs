use hitable::{HitRecord, Hitable};
use ray::Ray;
use aabb::{AABB, surrounding_box};

#[derive(Clone)]
pub struct HitableList {
    list: Vec<Box<Hitable>>
}

impl HitableList {
    pub fn new(list: Vec<Box<Hitable>>) -> HitableList {
        HitableList{
            list: list
        }
    }
}

impl Hitable for HitableList {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let mut rec: Option<HitRecord> = None;
        let mut closest = tmax;
        for elem in self.list.iter() {
            if let Some(tmp_rec) = elem.hit(&ray, tmin, closest) {
                closest = tmp_rec.t;
                rec = Some(tmp_rec);
            }
        }
        rec
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        if self.list.len() >= 1 {
            if let Some(first) = self.list[0].bounding_box(t0, t1) {
                let mut b = first;
                for elem in (&self.list[1..]).iter() {
                    if let Some(tmp) = elem.bounding_box(t0, t1) {
                        b = surrounding_box(b, tmp);
                    }
                }
                return Some(b);
            }
        }
        None
    }

    fn box_clone(&self) -> Box<Hitable> {
        Box::new((*self).clone())
    }
}
