use cgmath::Vector3;
use crate::texture::Texture;

pub struct SolidColorTexture {
    pub color: Vector3<f64>,
}

impl Texture for SolidColorTexture {
    fn value(&self, u: f64, v: f64, p: Vector3<f64>) -> Vector3<f64> {
        return self.color;
    }
}