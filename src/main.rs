mod vec3;
mod ray;

use vec3::{Vec3, unit_vector};
use ray::Ray;

fn color(ray: Ray) -> Vec3 {
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
    for j in (0..ny - 1).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let ray = Ray{
                origin: origin, 
                direction:lower_left_corner + u * horizontal + v * vertical
            };
            let col = color(ray);
            let ir = (255.99 * col.x) as i32;
            let ig = (255.99 * col.y) as i32;
            let ib = (255.99 * col.z) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
