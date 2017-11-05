use vec3::Vec3;

pub trait Texture : Sync + Send {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3;
}

pub struct ConstantTexture {
    color: Vec3
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
