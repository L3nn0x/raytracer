use vec3::Vec3;
use perlin::Perlin;

extern crate image;

use image::{open, RgbImage};

pub trait Texture : Sync + Send {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3;
}

pub struct ConstantTexture {
    color: Vec3
}

pub struct CheckerTexture {
    odd: Box<Texture>,
    even: Box<Texture>
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64
}

pub struct ImageTexture {
    image: RgbImage
}

impl ImageTexture {
    pub fn new(path: &str) -> ImageTexture {
        ImageTexture{
            image: open(&path).unwrap().to_rgb()
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, _p: Vec3) -> Vec3 {
        let i = u * self.image.width() as f32;
        let j = (1.0 - v) * self.image.height() as f32 - 0.001;
        let i = if i < 0.0 { 0.0 } else { i };
        let j = if j < 0.0 { 0.0 } else { j };
        let i = if i > (self.image.width() - 1) as f32 { (self.image.width() - 1) as f32 } else { i };
        let j = if j > (self.image.height() - 1) as f32 { (self.image.height() - 1) as f32 } else { j };
        let pixel = self.image.get_pixel(i as u32, j as u32);
        Vec3::new(pixel.data[0] as f64 / 255.0, pixel.data[1] as f64 / 255.0, pixel.data[2] as f64 / 255.0)
    }
}

impl NoiseTexture {
    pub fn new(scale: f64) -> NoiseTexture {
        NoiseTexture{noise: Perlin::new(), scale: scale}
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f32, _v: f32, p: Vec3) -> Vec3 {
        //Vec3::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + self.noise.noise(self.scale * p))
        //Vec3::new(1.0, 1.0, 1.0) * self.noise.turb(self.scale * p, 7)
        Vec3::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + (self.scale * p.x + 10.0 * self.noise.turb(p, 7)).sin())
    }
}

impl ConstantTexture {
    pub fn new(color: Vec3) -> ConstantTexture {
        ConstantTexture{
            color: color
        }
    }
}

impl Texture for ConstantTexture {
    fn value(&self, _u: f32, _v: f32, _p: Vec3) -> Vec3 {
        self.color.clone()
    }
}

impl CheckerTexture {
    pub fn new(odd: Box<Texture>, even: Box<Texture>) -> CheckerTexture {
        CheckerTexture{
            odd: odd,
            even: even
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
