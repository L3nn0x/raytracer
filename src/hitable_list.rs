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
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32, rec: &mut HitRecord) -> bool {
        let mut tmp_rec: HitRecord = Default::default();
        let mut hit_anything = false;
        let mut closest = tmax;
        for elem in self.list.iter() {
            if elem.hit(&ray, tmin, closest, &mut tmp_rec) {
                hit_anything = true;
                closest = tmp_rec.t;
                *rec = tmp_rec.clone();
            }
        }
        hit_anything
    }
}
