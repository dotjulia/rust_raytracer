use crate::color::Color;

pub trait Output {
    fn save (&self, path: &str);
    fn set_pixel(&mut self, x: usize, y: usize, c: Color);
}