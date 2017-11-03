mod vec3;
mod ray;
mod hitable;
mod sphere;
mod hitable_list;
mod camera;

use vec3::Vec3;
use ray::Ray;
use hitable_list::HitableList;
use sphere::Sphere;
use hitable::{HitRecord, Hitable};
use camera::Camera;

extern crate rand;

fn color(ray: Ray, world: &Hitable) -> Vec3 {
    let mut rec : HitRecord = Default::default();
    if world.hit(&ray, 0.0, 10000.0, &mut rec) {
        return 0.5 * (rec.normal + Vec3::new(1.0, 1.0, 1.0));
    }
    let u = ray.direction();
    let t = 0.5 * (u.y + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) +
        t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;
    println!("P3\n{} {}\n255", nx, ny);
    let objs: Vec<Box<Hitable>> = vec![
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0))
    ];
    let world = HitableList::new(objs);
    let cam = Camera::new();
    for j in (0..ny - 1).rev() {
        for i in 0..nx {
            let mut col: Vec3 = Default::default();
            for _k in 0..ns {
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
