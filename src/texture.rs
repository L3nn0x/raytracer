use vec3::Vec3;

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

impl ConstantTexture {
    pub fn new(color: Vec3) -> ConstantTexture {
        ConstantTexture{
            color: color
        }
    }
}

impl Texture for ConstantTexture {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
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