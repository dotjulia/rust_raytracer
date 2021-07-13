use cgmath::{Vector3, Vector2};

pub type UVCoords = Vector2<f64>;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: Vector3<f64>) -> Vector3<f64>;
}