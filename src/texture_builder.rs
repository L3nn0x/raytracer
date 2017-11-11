use texture::{Texture, ConstantTexture, CheckerTexture, NoiseTexture, ImageTexture};
use vec3::Vec3;

use std::sync::Arc;

pub enum TextureType<'a> {
    Constant(Vec3),
    Checker(Box<Texture>, Box<Texture>),
    Noise(f64),
    Image(&'a str)
}

pub fn box_texture_builder(t: TextureType) -> Box<Texture> {
    match t {
        TextureType::Constant(v) => Box::new(ConstantTexture::new(v)),
        TextureType::Checker(a, b) => Box::new(CheckerTexture::new(a, b)),
        TextureType::Noise(s) => Box::new(NoiseTexture::new(s)),
        TextureType::Image(s) => Box::new(ImageTexture::new(s)),
    }
}

pub fn texture_builder(t: TextureType) -> Arc<Texture> {
    match t {
        TextureType::Constant(v) => Arc::new(ConstantTexture::new(v)),
        TextureType::Checker(a, b) => Arc::new(CheckerTexture::new(a, b)),
        TextureType::Noise(s) => Arc::new(NoiseTexture::new(s)),
        TextureType::Image(s) => Arc::new(ImageTexture::new(s)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_texture_builder() {
        let a = box_texture_builder(TextureType::Constant(Default::default()));
        let b = box_texture_builder(TextureType::Constant(Default::default()));
        let _c = box_texture_builder(TextureType::Checker(a, b));
        let _d = box_texture_builder(TextureType::Noise(0.0));
    }
}
