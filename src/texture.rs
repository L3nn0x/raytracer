use vec3::Vec3;
use perlin::Perlin;

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
    noise: Perlin
}

impl NoiseTexture {
    pub fn new() -> NoiseTexture {
        NoiseTexture{noise: Perlin::new()}
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f32, _v: f32, p: Vec3) -> Vec3 {
        Vec3::new(1.0, 1.0, 1.0) * self.noise.noise(p)
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
