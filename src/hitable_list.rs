use hitable::{HitRecord, Hitable};
use vec3::Vec3;
use ray::Ray;

pub struct HitableList {
    pub list: Vec<Box<Hitable>>
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
