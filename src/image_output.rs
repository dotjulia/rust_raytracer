extern crate image;

use crate::color::Color;
use image::{Rgb, ImageBuffer};

//use std::cmp;

pub struct ImageOutput {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Vec<Color>>,
}

impl ImageOutput {
    pub fn new (width: u32, height: u32) -> ImageOutput {
        return ImageOutput {
            height,
            width,
            pixels: vec![vec![Color{r: 0.0, g: 0.0, b: 0.0}; width as usize]; height as usize],
        };
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, c: Color) {
        self.pixels[y][x] = c;
    }

    pub fn remap_pixel_range(&mut self, max_brightness: f64) {
        let mut max_val = 0.0;
        for i in self.pixels.iter() { //get max val
            for j in i.iter() {
                if j.r > max_val {
                    max_val = j.r;
                }
                if j.g > max_val {
                    max_val = j.g;
                }
                if j.b > max_val {
                    max_val = j.b;
                }
            }
        }
        if max_val > max_brightness {
            max_val = max_brightness;
        }
        //println!("{}", max_val);
        for mut i in self.pixels.iter_mut() { //remap pixel between 0 and 1
            for mut j in i.iter_mut() {
                j.r = if j.r / max_val > 1.0 {1.0} else {j.r/max_val};
                j.g = if j.g / max_val > 1.0 {1.0} else {j.g/max_val};
                j.b = if j.b / max_val > 1.0 {1.0} else {j.b/max_val};
            }
        }
    }

    pub fn save (&self, path: &str) {
        let mut imgbuf = ImageBuffer::new(self.width, self.height);
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            //cmp::min(cmp::max(self.pixels.get(y).get(x)*255, 0).r, 255)
            *pixel = Rgb([(self.pixels[y as usize][x as usize].r.sqrt()*255.0) as u8, (self.pixels[y as usize][x as usize].g.sqrt()*255.0) as u8, (self.pixels[y as usize][x as usize].b.sqrt()*255.0) as u8]);
        }
        imgbuf.save(path).unwrap();
    }
}