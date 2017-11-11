use hitable::Hitable;
use sphere::{Sphere, MovingSphere};
use hitable_list::HitableList;
use bvh::BVHNode;
use vec3::Vec3;
use std::sync::Arc;
use material::Material;

pub enum HitableType {
    Sphere(Vec3, f64, Arc<Material>),
    MovingSphere(Vec3, Vec3, f64, f32, f32, Arc<Material>),
    HitableList(Vec<Box<Hitable>>),
    Bvh(Vec<Box<Hitable>>, f32, f32)
}

pub fn hitable_builder(t: HitableType) -> Box<Hitable> {
    match t {
        HitableType::Sphere(c, r, m) => Box::new(Sphere::new(c, r, m)),
        HitableType::MovingSphere(c0, c1, r, t0, t1, m) => Box::new(MovingSphere::new(c0, c1, r, t0, t1, m)),
        HitableType::HitableList(v) => Box::new(HitableList::new(v)),
        HitableType::Bvh(v, t0, t1) => Box::new(BVHNode::new(v, t0, t1)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hitable_builder() {
        use material_builder::{MaterialType, material_builder};
        let m = material_builder(MaterialType::Metal(Default::default(), 0.0));
        let _a = hitable_builder(HitableType::MovingSphere(Default::default(), Default::default(), 0.0, 0.0, 0.0, m));
        let _a = ::std::panic::catch_unwind(|| hitable_builder(HitableType::Bvh(Vec::new(), 0.0, 0.0)));
    }
}
