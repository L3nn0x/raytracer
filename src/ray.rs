use vec3::Vec3;


#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}

#[test]
fn ray_point_at_parameter() {
    let r = Ray{
        origin: Vec3{x: 0.0, y: 0.0, z: 0.0},
        direction: Vec3{x: 1.0, y: 0.0, z: 0.0}
    };
    let p = r.point_at_parameter(2.0);
    assert!(p == Vec3{x: 2.0, y: 0.0, z: 0.0});
}
