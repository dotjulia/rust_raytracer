use crate::scene::Scene;
use crate::ray::Ray;
use crate::image_output::ImageOutput;
use crate::color::Color;
use cgmath::{Vector3, InnerSpace};
use crate::hitable::{Hitable, HitRecord};
extern crate rand;
use std::thread;

use self::rand::Rng;
use std::sync::Arc;

pub struct RenderEngine{
}

impl RenderEngine {
    pub fn render(scene: Box<Scene>, image: &mut Box<ImageOutput>, max_depth: i32) {
        let ns = scene.camera.antialiasing_iterations;
        let mut curr_line = 0;
        //let mut join_handles = Vec::with_capacity(scene.height as usize);
        println!();
        for j in (0..scene.height).rev() {
            let width = scene.width;
            let height = scene.height;
            let world = scene.world.duplicate();
                let mut rng = rand::thread_rng();
                for i in 0..width {
                    let mut color = Color {r: 0.0, g: 0.0, b: 0.0};
                    for s in 0..ns {
                        let u = (i as f64 + ( if ns>1 { rng.gen::<f64>() } else { 0.0 })) as f64 / width as f64;
                        let v = (j as f64 + ( if ns>1 { rng.gen::<f64>() } else { 0.0 })) as f64 / height as f64;
                        let r = scene.camera.get_ray(u, v);
                        color = color + RenderEngine::color_at(r, &world, 0, max_depth);
                    }
                    color /= ns as f64;
                    image.set_pixel(i as usize, j as usize, color);
                }
            curr_line += 1;
            println!("{}%", curr_line*100/scene.height);
        }
    }

    // fn hit_sphere(center: Vector3<f64>, radius: f64, r: &Ray) -> f64 {
    //     let oc = r.origin - center;
    //     let a = r.direction.dot(r.direction);
    //     let b = oc.dot(r.direction) * 2.0;
    //     let c = oc.dot(oc) - radius * radius;
    //     let discriminant = b * b - 4.0 * a * c;
    //     if discriminant < 0.0 {
    //         return -1.0;
    //     } else {
    //         return (-b - discriminant.sqrt()) / (2.0 * a);
    //     }
    // }

    fn color_at(ray: Ray, world: &Box<dyn Hitable>, depth: i32, max_depth: i32) -> Color {
        /*let t = RenderEngine::hit_sphere(Vector3::new(0.0, 0.0, -1.0), 0.5, &ray);
        if t > 0.0 {
            let n = (ray.point_at_parameter(t) - Vector3::new(0.0,0.0,-1.0)).normalize();
            let ret_val = Vector3::new(n.x + 1.0, n.y + 1.0, n.z + 1.0);
            return Color{r: ret_val.x, g: ret_val.y, b: ret_val.z};
        }
        let unit_direction = ray.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        let ret_vec = (Vector3::new(1.0, 1.0, 1.0)) * (1.0 - t) + (Vector3::new(0.5, 0.7, 1.0)) * t;
        return Color{r: ret_vec.x, g: ret_vec.y, b: ret_vec.z};*/
        let mut rec = HitRecord::new_empty();
        if world.hit(&ray, 0.001, 100000000.0, & mut rec) {
            let mut scattered = Ray::new_empty();
            let mut attenuation = Vector3::new(0.0, 0.0, 0.0);
            let do_scatter = rec.material.scatter(&ray, &rec, &mut attenuation, &mut scattered);
            if depth < max_depth && do_scatter {
                //println!("Scattered: {}, Attenuation: Vector3 [{}, {}, {}]", scattered, attenuation);
                return RenderEngine::color_at(scattered, world, depth+1, max_depth) * attenuation;
            } else {
                return Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                }
            }
            //return RenderEngine::color_at(Ray {origin: rec.position, direction: target - rec.position}, world) * 0.5;
            //return Color::from_vector3(Vector3::new(rec.normal.x + 1.0, rec.normal.y + 1.0, rec.normal.z + 1.0) * 0.5);
        } else {
            let unit_direction = ray.direction.normalize();
            let t = (unit_direction.y + 1.0) * 0.5;
            return Color::from_vector3(Vector3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vector3::new(0.5, 0.7, 1.0) * t);
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