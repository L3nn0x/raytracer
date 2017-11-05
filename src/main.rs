mod vec3;
mod ray;
mod hitable;
mod sphere;
mod hitable_list;
mod camera;
mod material;
mod lambertian;
mod metal;
mod dielectric;

use vec3::Vec3;
use ray::Ray;
use hitable_list::HitableList;
use sphere::Sphere;
use hitable::Hitable;
use camera::Camera;
use lambertian::Lambertian;
use metal::Metal;
use dielectric::Dielectric;

use std::rc::Rc;
use std::f64::consts::PI;

extern crate rand;

fn color(ray: Ray, world: &Hitable, depth: i32) -> Vec3 {
    if let Some(rec) = world.hit(&ray, 0.001, 10000.0) {
        if depth < 50 {
            if let Some(result) = rec.mat.scatter(&ray, &rec) {
                return result.attenuation * color(result.scattered, world, depth + 1);
            } else {
                return Vec3::new(0.0, 0.0, 0.0);
            }
        }
    }
    let u = ray.direction();
    let t = 0.5 * (u.y + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 10;
    println!("P3\n{} {}\n255", nx, ny);
    let R = (PI / 4.0).cos();
    let objs: Vec<Box<Hitable>> = vec![
        Box::new(Sphere::new(Vec3::new(-R, 0.0, -1.0), R, Rc::new(Lambertian::new(Vec3::new(0.0, 0.0, 1.0))))),
        Box::new(Sphere::new(Vec3::new(R, 0.0, -1.0), R, Rc::new(Lambertian::new(Vec3::new(1.0, 0.0, 0.0))))),
    ];
    let world = HitableList::new(objs);
    let cam = Camera::new(90.0, nx as f64 / ny as f64);
    for j in (0..ny - 1).rev() {
        for i in 0..nx {
            let mut col: Vec3 = Default::default();
            for _k in 0..ns {
                let u = (rand::random::<f64>() + i as f64) / nx as f64;
                let v = (rand::random::<f64>() + j as f64) / ny as f64;
                let ray = cam.get_ray(u, v);
                col += color(ray, &world, 0);
            }
            col /= ns as f64;
            let col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());
            let ir = (255.99 * col.x) as i32;
            let ig = (255.99 * col.y) as i32;
            let ib = (255.99 * col.z) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
