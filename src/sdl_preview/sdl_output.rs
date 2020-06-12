use crate::trait_output::Output;
use crate::color::Color;
use sdl2::EventPump;
use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::{Point, Rect};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::render_information::RenderInformation;
use sdl2::pixels::PixelFormatEnum;
use rand::thread_rng;
use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;

pub struct SDLOutput {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Vec<Color>>,
    sdl_thread: Option<JoinHandle<()>>,
    send_vec: Vec<Sender<SetPixelResultRow>>,
}

impl Output for SDLOutput {
    fn save(&mut self, path: &str, info: RenderInformation) {
        self.render_end_window();
        self.sdl_thread.take().unwrap().join();
    }

    fn set_pixel(&mut self, x: usize, y: usize, c: Color) {
        self.pixels[y][x] = c;
        // self.send_vec.pop().unwrap().send(SetPixelResult {
        //     x,
        //     y,
        //     c,
        //     is_real_value: true,
        // });
        //self.texture.update(Rect::new(x as i32, y as i32, 1, 1), &[255u8], 0);
        //self.canvas.set_draw_color(c.to_sdl_color());
        //self.canvas.draw_point(Point::new(x as i32,y as i32));
        //self.canvas.present();
    }

    fn set_row(&mut self, y: usize, c: Vec<Color>) {
        self.send_vec.pop().unwrap().send(SetPixelResultRow {
            y,
            c,
            is_real_value: true,
        });
    }

    fn wants_row(&self) -> bool {
        return true;
    }
}

struct SetPixelResultRow {
    y: usize,
    c: Vec<Color>,
    is_real_value: bool,
}

impl SDLOutput {

    fn render_end_window(&mut self) -> Result<(), String> {
        // let texture_creator = self.canvas.texture_creator();
        // let mut texture = texture_creator
        //     .create_texture_streaming(PixelFormatEnum::RGB24, 256, 256)
        //     .map_err(|e| e.to_string())?;
        //
        // texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        //     for y in 0..self.height as usize {
        //         for x in 0..self.width as usize {
        //             let offset = y * pitch + x * 3;
        //             buffer[offset] = (self.pixels[y][x].r * 255.0) as u8;
        //             buffer[offset + 1] = (self.pixels[y][x].g * 255.0) as u8;
        //             buffer[offset + 2] = (self.pixels[y][x].b * 255.0) as u8;
        //         }
        //     }
        // });
        //
        // self.canvas.clear();
        // self.canvas.copy(&texture, None, Some(Rect::new(0,0, self.width, self.height)))?;
        // self.canvas.copy_ex(&texture, None, Some(Rect::new(0, 0, self.width, self.height)), 0.0, None, false, false)?;
        // self.canvas.present();
        //
        // 'running: loop {
        //     for event in self.event_pump.poll_iter() {
        //         match event {
        //             Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
        //                 break 'running;
        //             },
        //             _ => {},
        //         }
        //     }
        //     self.canvas.clear();
        // }
        Ok(())
    }

    fn start_sdl_window(width: u32, height: u32, rx: Receiver<SetPixelResultRow>) -> Result<(), String> {
        println!("SDL thread started!");
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem.window("Preview", width, height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let mut event_pump = sdl_context.event_pump()?;

        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        let texture_creator = canvas.texture_creator();

        //window.surface(&event_pump).unwrap().fill_rect(Rect::new(0,0, 10, 10), Color::RGB(255,0,0).to_sdl_color()).unwrap();

        let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGB24, width, 1)
            .map_err(|e| e.to_string())?;
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..}
                    | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    Event::KeyDown { keycode: Some(Keycode::S), repeat: false, ..} => {
                        canvas.present();
                    },
                    _ => {}
                }
            }
            //let mut surface = window.surface(&event_pump)?;
            let mut r = rx.try_recv().unwrap_or(SetPixelResultRow {
                y: 0,
                c: vec![Color::RGB(0,0,0)],
                is_real_value: false,
            });
            while r.is_real_value {
                texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
                    for x in 0..width {
                        let offset = x as usize * 3;
                        buffer[offset] = (r.c[x as usize].r * 255.0) as u8;
                        buffer[offset + 1] = (r.c[x as usize].g * 255.0) as u8;
                        buffer[offset + 2] = (r.c[x as usize].b * 255.0) as u8;
                    }
                })?;
                canvas.copy(&texture, None, Some(Rect::new(0, r.y as i32, width, 1)))?;
                canvas.present();
                //println!("{}", r.y);
                r = rx.try_recv().unwrap_or(SetPixelResultRow {
                    y: 0,
                    c: vec![Color::RGB(0,0,0)],
                    is_real_value: false,
                });
            }
            //surface.finish();
        }
        Ok(())
    }

    pub fn new(width: u32, height: u32) -> SDLOutput {

        let (tx, rx) = std::sync::mpsc::channel::<SetPixelResultRow>();
        let mut send_vec = Vec::<Sender<SetPixelResultRow>>::with_capacity(width as usize * height as usize);
        for y in 0..height {
            send_vec.push(tx.clone());
        }
        let sdl_thread = thread::spawn(move || {
            SDLOutput::start_sdl_window(width, height, rx);
        });
        return SDLOutput {
            width,
            height,
            pixels: vec![vec![Color{r: 0.0, g: 0.0, b: 0.0}; width as usize]; height as usize],
            sdl_thread: Option::from(sdl_thread),
            send_vec,
        };
    }
}