use crate::texture::Texture;
use dyn_clone::private::sync::Arc;
use cgmath::Vector3;
use crate::solid_color_texture::SolidColorTexture;

pub struct CheckerTexture {
    odd: Arc<dyn Texture>,
    even: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(odd: Vector3<f64>, even: Vector3<f64>) -> CheckerTexture {
        return CheckerTexture {
            odd: Arc::from(SolidColorTexture {
                color: odd,
            }),
            even: Arc::from(SolidColorTexture {
                color: even,
            }),
        };
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Vector3<f64>) -> Vector3<f64> {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        return if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}