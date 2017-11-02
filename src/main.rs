mod vec3;
mod ray;
mod hitable;
mod sphere;
mod hitable_list;

use vec3::{Vec3, unit_vector};
use ray::Ray;
use hitable_list::HitableList;
use sphere::Sphere;
use hitable::{HitRecord, Hitable};

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
    for j in (0..ny - 1).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let ray = Ray{
                origin: origin, 
                direction:lower_left_corner + u * horizontal + v * vertical
            };
            let col = color(ray, &world);
            let ir = (255.99 * col.x) as i32;
            let ig = (255.99 * col.y) as i32;
            let ib = (255.99 * col.z) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
