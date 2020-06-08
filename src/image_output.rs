extern crate image;

//use std::cmp;

pub struct Image {
    width: u16,
    height: u16,
    pub pixels: Vec<Vec<Color>>,
}

impl Image {
    pub fn new (width: u16, height: u16) -> Image {
        return Image {
            height,
            width,
            pixels: vec![[Color{r: 0.0,g: 0.0,b: 0.0}; width as usize]; height as usize],
        };
    }

    pub fn set_pixel(x: u16, height: u16, c: Color) {
        pixels[y][x] = c;
    }

    pub fn save (&self, path: string) {
        let mut imgbuf = ImageBuffer::new(self.height, self.width);
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            //cmp::min(cmp::max(self.pixels.get(y).get(x)*255, 0).r, 255)
            *pixel = Rgb([self.pixels[y][x].r*255, self.pixels[y][x].g*255, self.pixels[y][x].b*255]);
        }
        imgbuf.save(path).unwrap();
    }
}