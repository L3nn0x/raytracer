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
mod aabb;
mod bvh;
mod texture;

use vec3::{Vec3, unit_vector};
use ray::Ray;
use hitable_list::HitableList;
use sphere::{Sphere, MovingSphere};
use hitable::Hitable;
use camera::Camera;
use lambertian::Lambertian;
use metal::Metal;
use dielectric::Dielectric;
use texture::{ConstantTexture, CheckerTexture};

use std::sync::Arc;

extern crate image;
extern crate rand;
extern crate rayon;

use image::Rgb;

use rayon::prelude::*;

fn random_scene() -> HitableList {
    let mut objs: Vec<Box<Hitable>> = vec![
        Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Arc::new(Lambertian::new(
                        Box::new(CheckerTexture::new(
                                Box::new(ConstantTexture::new(Vec3::new(0.2, 0.3, 0.1))),
                                Box::new(ConstantTexture::new(Vec3::new(0.9, 0.9, 0.9)))
                                ))))))
    ];
    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3::new(a as f64 + 0.9 * rand::random::<f64>(), 0.2, b as f64 + 0.9 * rand::random::<f64>());
            let center_end = center + Vec3::new(0.0, 0.5 * rand::random::<f64>(), 0.0);
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let rd = rand::random::<f32>();
                let obj:Box<Hitable> = if rd < 0.8 { // diffuse
                    Box::new(MovingSphere::new(center, center_end, 0.2, 0.0, 1.0,
                    Arc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(
                            rand::random::<f64>() * rand::random::<f64>(),
                            rand::random::<f64>() * rand::random::<f64>(),
                            rand::random::<f64>() * rand::random::<f64>())))))))
                } else if rd < 0.95 { // metal
                    Box::new(Sphere::new(center, 0.2, Arc::new(Metal::new(Vec3::new(
                            0.5 * (1.0 + rand::random::<f64>()),
                            0.5 * (1.0 + rand::random::<f64>()),
                            0.5 * (1.0 + rand::random::<f64>())),
                            0.5 * rand::random::<f32>()))))
                } else { // glass
                    Box::new(Sphere::new(center, 0.2, Arc::new(Dielectric::new(1.5))))
                };
                objs.push(obj);
            }
        }
    }
    objs.push(Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Arc::new(Dielectric::new(1.5)))));
    objs.push(Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, Arc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(0.4, 0.2, 0.1))))))));
    objs.push(Box::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)))));
    HitableList::new(objs)
}

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
    let u = unit_vector(ray.direction);
    let t = 0.5 * (u.y + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    let nx = 1200;
    let ny = 800;
    let ns = 10;
    /*let objs: Vec<Box<Hitable>> = vec![
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Arc::new(
                    Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(0.1, 0.2, 0.5))))))),
					Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Arc::new(
                    Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(0.8, 0.8, 0.0))))))),
        Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.2)))),
        Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, Arc::new(Dielectric::new(1.5)))),
        Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.45, Arc::new(Dielectric::new(1.5)))),
    ];
    let world = HitableList::new(objs);*/
    let world = random_scene();
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let cam = Camera::new(look_from, look_at, Vec3::new(0.0, 1.0, 0.0), 20.0,
                        nx as f64 / ny as f64, aperture, dist_to_focus, 0.0, 1.0);
    let mut pixels = vec![Rgb { data: [0; 3] }; nx * ny];
    pixels.par_iter_mut().enumerate().for_each(|(idx, c)| {
        let i = idx % nx;
        let j = ny - 1 - idx / nx;

        let mut col: Vec3 = Default::default();
        for _k in 0..ns {
            let u = (rand::random::<f64>() + i as f64) / nx as f64;
            let v = (rand::random::<f64>() + j as f64) / ny as f64;
            let ray = cam.get_ray(u, v);
            col += color(ray, &world, 0);
        }
        col /= ns as f64;
        let col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());
        let ir = (255.99 * col.x) as u8;
        let ig = (255.99 * col.y) as u8;
        let ib = (255.99 * col.z) as u8;
        *c = Rgb {
            data: [ir, ig, ib],
        };
    });
    let mut image = image::ImageBuffer::new(nx as u32, ny as u32);
    for j in 0..ny {
        for i in 0..nx {
            image.put_pixel(i as u32, j as u32, pixels[i + j * nx]);
        }
    }
    image.save("img.png").unwrap();
}
