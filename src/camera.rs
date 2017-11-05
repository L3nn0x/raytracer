use vec3::{Vec3, cross, dot};
use ray::Ray;

use std::f64::consts::PI;

extern crate rand;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, vup: Vec3, vfov: f64, aspect: f64, aperture: f64, focus_distance: f64) -> Camera {
        let theta = vfov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (look_from - look_at).make_unit();
        let u = cross(&vup, &w).make_unit();
        let v = cross(&w, &u);
        Camera{
            origin: look_from,
            lower_left_corner: look_from - half_width * focus_distance * u - half_height * focus_distance * v - focus_distance * w,
            horizontal: 2.0 * half_width * focus_distance * u,
            vertical: 2.0 * half_height * focus_distance * v,
            u: u,
            v: v,
            lens_radius: aperture / 2.0
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(self.origin + offset,
                self.lower_left_corner +
                s * self.horizontal +
                t * self.vertical - self.origin - offset)
    }
}

fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = 2.0 * Vec3::new(rand::random::<f64>(), rand::random::<f64>(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
        if dot(&p, &p) < 1.0 {
            return p;
        }
    }
}
