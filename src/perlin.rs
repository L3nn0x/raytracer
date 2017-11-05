use vec3::Vec3;

extern crate rand;

use rand::{random, thread_rng, Rng};

pub struct Perlin {
    ranfloat: [f64; 256],
    perm_x: [u8; 256],
    perm_y: [u8; 256],
    perm_z: [u8; 256]
}

impl Perlin {
    pub fn noise(&self, p: Vec3) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();
        let i = ((4.0 * p.x) as i32 & 255) as usize;
        let j = ((4.0 * p.y) as i32 & 255) as usize;
        let k = ((4.0 * p.z) as i32 & 255) as usize;
        self.ranfloat[(self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]) as usize]
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

fn perlin_generate() -> [f64; 256] {
    let mut p = [0.0; 256];
    for i in 0..256 {
        p[i] = rand::random::<f64>();
    }
    p
}

fn perlin_generate_perm() -> [u8; 256] {
    let mut p = [0; 256];
    for i in 0..256 {
        p[i] = i as u8;
    }
    let mut rng = thread_rng();
    rng.shuffle(&mut p);
    p
}
