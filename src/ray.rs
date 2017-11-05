use vec3::Vec3;


pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub time: f32
}

impl Ray {
    pub fn point_at_parameter(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }

    pub fn new(origin: Vec3, direction: Vec3, ti: f32) -> Ray {
        Ray{
            origin: origin,
            direction: direction,
            time: ti
        }
    }
}

#[test]
fn ray_point_at_parameter() {
    let r = Ray::new(Default::default(), Vec3::new(1.0, 0.0, 0.0), 0.0);
    let p = r.point_at_parameter(2.0);
    assert!(p == Vec3::new(2.0, 0.0, 0.0));
}
