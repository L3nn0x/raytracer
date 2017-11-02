mod vec3;
mod ray;
mod hitable;
mod sphere;
mod hitable_list;
mod camera;

use vec3::{Vec3, unit_vector};
use ray::Ray;
use hitable_list::HitableList;
use sphere::Sphere;
use hitable::{HitRecord, Hitable};
use camera::Camera;

extern crate rand;

fn color(ray: Ray, world: &Hitable) -> Vec3 {
    let mut rec : HitRecord = Default::default();
    if world.hit(&ray, 0.0, 10000.0, &mut rec) {
        return 0.5 * (rec.normal + Vec3{x: 1.0, y: 1.0, z: 1.0});
    }
    let u = unit_vector(&ray.direction);
    let t = 0.5 * (u.y + 1.0);
    (1.0 - t) * Vec3{x: 1.0, y: 1.0, z: 1.0} + t * Vec3{x: 0.5, y: 0.7, z: 1.0}
}

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;
    println!("P3\n{} {}\n255", nx, ny);
    let origin = Vec3{x: 0.0, y: 0.0, z: 0.0};
    let lower_left_corner = Vec3{x: -2.0, y: -1.0, z: -1.0};
    let horizontal = Vec3{x: 4.0, y: 0.0, z: 0.0};
    let vertical = Vec3{x: 0.0, y: 2.0, z: 0.0};
    let objs: Vec<Box<Hitable>> = vec![
        Box::new(Sphere{center: Vec3{x: 0.0, y: 0.0, z: -1.0},
                        radius: 0.5}),
        Box::new(Sphere{center: Vec3{x: 0.0, y: -100.5, z: -1.0},
                        radius: 100.0})
    ];
    let world = HitableList{list: objs};
    let cam: Camera = Default::default();
    for j in (0..ny - 1).rev() {
        for i in 0..nx {
            let mut col: Vec3 = Default::default();
            for k in 0..ns {
                let u = rand::random::<f32>() + i as f32 / nx as f32;
                let v = rand::random::<f32>() + j as f32 / ny as f32;
                let ray = cam.get_ray(u, v);
                col += color(ray, &world);
            }
            col /= ns as f32;
            let ir = (255.99 * col.x) as i32;
            let ig = (255.99 * col.y) as i32;
            let ib = (255.99 * col.z) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
