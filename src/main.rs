mod vec3;
mod ray;

use vec3::{Vec3, unit_vector, dot};
use ray::Ray;

fn hit_sphere(center: Vec3, radius: f32, ray: &Ray) -> f32 {
    let oc = ray.origin - center;
    let a = dot(&ray.direction, &ray.direction);
    let b = 2.0 * dot(&oc, &ray.direction);
    let c = dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return -1.0;
    }
    (-b - discriminant.sqrt()) / (2.0 * a)
}

fn color(ray: Ray) -> Vec3 {
    let t = hit_sphere(Vec3{x: 0.0, y: 0.0, z: -1.0}, 0.5, &ray);
    if t > 0.0 {
        let n = unit_vector(&(ray.point_at_parameter(t) - Vec3{x: 0.0, y: 0.0, z: -1.0}));
        return 0.5 * (n + Vec3{x: 1.0, y: 1.0, z: 1.0});
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
