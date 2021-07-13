use crate::scene::Scene;
use crate::ray::Ray;
use crate::image_output::ImageOutput;
use crate::color::Color;
use cgmath::{Vector3, InnerSpace};
use crate::hitable::{Hitable, HitRecord};
extern crate rand;
use std::thread;
use threadpool::ThreadPool;

use self::rand::{Rng, thread_rng};
use std::sync::Arc;
use std::sync::mpsc::Sender;
use core::fmt;
use std::fmt::Formatter;
use colored::Colorize;
use crate::trait_output::Output;
use self::rand::seq::SliceRandom;

pub struct RenderEngineMultithread{
}

struct RenderThreadResult {
    row: u32,
    pixel: Vec<Color>,
}

impl fmt::Display for RenderThreadResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.row, self.pixel.len())
    }
}

#[cfg(debug_assertions)]
fn check_debug() {
    println!("{}", "Running in debug mode. Rendering will be significantly slower.".red());
}

#[cfg(not(debug_assertions))]
fn check_debug() {
    // Debugging disabled
}

fn lines_as_arr (lines: u32) -> Vec<u32> {
    let mut arr = Vec::with_capacity(lines as usize);
    for i in 0..lines {
        arr.push(i);
    }
    return arr;
}

impl RenderEngineMultithread {
    pub fn render(scene: Box<Scene>, image: &mut Box<dyn Output>, max_depth: i32, n_workers: usize, randomize_line_order: bool) {
        check_debug(); // Checking if running in debug mode
        let ns = scene.camera.antialiasing_iterations;
        //let mut join_handles = Vec::with_capacity(scene.height as usize);
        let pool = ThreadPool::new(n_workers);
        let (tx, rx) = std::sync::mpsc::channel::<RenderThreadResult>();

        let mut rows = lines_as_arr(scene.height);
        if randomize_line_order {
            rows.shuffle(&mut thread_rng());
        }

        for j in rows {
            let width = scene.width;
            let height = scene.height;
            let world = scene.world.duplicate();
            let camera = scene.camera.duplicate();
            let tx = tx.clone();
            pool.execute(move || {
                let mut row: Vec<Color> = Vec::with_capacity(width as usize);
                for i in 0..width {
                    let mut rng = rand::thread_rng();
                    let mut color = Color {r: 0.0, g: 0.0, b: 0.0};
                    for s in 0..ns {
                        let u = (i as f64 + ( if ns>1 { rng.gen::<f64>() } else { 0.0 })) as f64 / width as f64;
                        let v = (j as f64 + ( if ns>1 { rng.gen::<f64>() } else { 0.0 })) as f64 / height as f64;
                        let r = camera.get_ray(u, v);
                        color = color + RenderEngineMultithread::color_at(r, &world, 0, max_depth);
                    }
                    color /= ns as f64;
                    row.push(color);
                    //image.set_pixel(i as usize, j as usize, color);
                }
                tx.send(RenderThreadResult {
                    row: j,
                    pixel: row,
                });
            });
        }
        let mut finished_rows = 0;
        for t in rx.iter() {
            if image.wants_row() {
                image.set_row(t.row as usize, t.pixel);
            } else {
                for (i, p) in t.pixel.iter().enumerate() {
                    image.set_pixel(i, t.row as usize, *p);
                }
            }
            finished_rows += 1;
            println!("{}/{}", finished_rows, scene.height);
            if finished_rows == scene.height {
                return;
            }
        }
    }

    fn color_at(ray: Ray, world: &Arc<dyn Hitable>, depth: i32, max_depth: i32) -> Color {
        let mut rec = HitRecord::new_empty();
        if world.hit(&ray, 0.001, 100000000.0, & mut rec) {
            let mut scattered = Ray::new_empty();
            let mut attenuation = Vector3::new(0.0, 0.0, 0.0);
            let mut emissive = false;
            let mut emission = Vector3::new(0.0, 0.0, 0.0);
            if depth < max_depth && rec.material.scatter(&ray, &rec, &mut attenuation, &mut scattered) {
                //println!("Scattered: {}, Attenuation: Vector3 [{}, {}, {}]", scattered, attenuation);
                return RenderEngineMultithread::color_at(scattered, world, depth+1, max_depth) * attenuation;
            } else {
                return Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                }
            }
       } else {
            let unit_direction = ray.direction.normalize();
            let t = (unit_direction.y + 1.0) * 0.5;
            return Color::from_vector3(Vector3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vector3::new(0.5, 0.7, 1.0) * t);
            /*return Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
            }*/
        }
    }

    pub fn random_in_unit_sphere() -> Vector3<f64> {
        let mut rng = rand::thread_rng();
        let mut p = Vector3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()) * 2.0 - Vector3::new(1.0, 1.0, 1.0);
        while p.magnitude2() >= 1.0 {
            p = Vector3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()) * 2.0 - Vector3::new(1.0, 1.0, 1.0);
        }
        return p;
    }
}