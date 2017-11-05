use hitable::{HitRecord, Hitable};
use ray::Ray;

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
}
