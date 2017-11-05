use vec3::{Vec3, dot, unit_vector};

extern crate rand;

use rand::{random, thread_rng, Rng};

pub struct Perlin {
    ranfloat: [Vec3; 256],
    perm_x: [u8; 256],
    perm_y: [u8; 256],
    perm_z: [u8; 256]
}

impl Perlin {
    pub fn noise(&self, p: Vec3) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();
        let u = u * u * (3.0 - 2.0 * u);
        let v = v * v * (3.0 - 2.0 * v);
        let w = w * w * (3.0 - 2.0 * w);
        let i = p.x.floor() as usize;
        let j = p.y.floor() as usize;
        let k = p.z.floor() as usize;
        let mut c = [[[Default::default(); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranfloat[(self.perm_x[(i + di) & 255] ^
                                               self.perm_y[(j + dj) & 255] ^
                                               self.perm_z[(k + dk) & 255]) as usize];
                }
            }
        }
        trilinear_interp(c, u, v, w)
    }

    pub fn new() -> Perlin {
        Perlin{
            ranfloat: perlin_generate(),
            perm_x: perlin_generate_perm(),
            perm_y: perlin_generate_perm(),
            perm_z: perlin_generate_perm(),
        }
    }
}

fn perlin_generate() -> [Vec3; 256] {
    let mut p = [Default::default(); 256];
    for i in 0..256 {
        p[i] = unit_vector(Vec3::new(-1.0 + 2.0 * rand::random::<f64>(),
                                    -1.0 + 2.0 * rand::random::<f64>(),
                                    -1.0 + 2.0 * rand::random::<f64>()));
    }
    p
}

fn permute(p: &mut [u8; 256]) {
    for i in (0..256).rev() {
        let target = (rand::random::<f64>() * (i + 1) as f64) as usize;
        p.swap(i, target);
    }
}

fn perlin_generate_perm() -> [u8; 256] {
    let mut p = [0; 256];
    for i in 0..256 {
        p[i] = i as u8;
    }
    permute(&mut p);
    p
}

fn trilinear_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);
    let mut accum: f64 = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let weight = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                accum += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu)) *
                         (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv)) *
                         (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww)) * dot(&c[i][j][k], &weight);
            }
        }
    }
    accum
}
