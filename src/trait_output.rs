use crate::color::Color;
use crate::render_information::RenderInformation;

pub trait Output {
    fn save (&mut self, path: &str, info: RenderInformation);
    fn set_pixel(&mut self, x: usize, y: usize, c: Color);
    fn set_row(&mut self, y: usize, c: Vec<Color>);
    fn wants_row(&self) -> bool;
}