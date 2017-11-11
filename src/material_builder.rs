use vec3::Vec3;
use material::Material;
use lambertian::Lambertian;
use dielectric::Dielectric;
use metal::Metal;
use texture::Texture;

use std::sync::Arc;

pub enum MaterialType {
    Lambertian(Arc<Texture>),
    Metal(Vec3, f32),
    Dielectric(f64)
}

pub fn material_builder(t: MaterialType) -> Arc<Material> {
    match t {
        MaterialType::Lambertian(t) => Arc::new(Lambertian::new(t)),
        MaterialType::Metal(a, f) => Arc::new(Metal::new(a, f)),
        MaterialType::Dielectric(idx) => Arc::new(Dielectric::new(idx)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_material_builder() {
        let _a = material_builder(MaterialType::Dielectric(0.0));
    }
}
