use vec3::Vec3;
use ray::Ray;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3
}

impl Default for Camera {
    fn default() -> Camera {
        Camera{
            origin: Default::default(),
            lower_left_corner: Vec3{x: -2.0, y: -1.0, z: -1.0},
            horizontal: Vec3{x: 4.0, y: 0.0, z: 0.0},
            vertical: Vec3{x: 0.0, y: 2.0, z: 0.0},
        }
    }
}

impl Camera {
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray{
            origin: self.origin,
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin
        }
    }
}
