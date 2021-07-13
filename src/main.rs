extern crate cgmath;
extern crate image;

mod image_output;
mod color;
mod scene;
mod ray;
mod hitable;
mod hitable_collection;
mod sphere;
mod camera;
mod material;
mod lambertian;
mod metal;
mod trait_output;
mod dielectric;
mod render_engine_multithread;
mod moving_sphere;
mod sdl_preview;
mod render_information;
mod AABB;
mod bvh_node;
mod texture;
mod solid_color_texture;

//use cgmath::{Vector3, InnerSpace};
//use image::{ImageBuffer, Rgb};
use crate::image_output::ImageOutput;
use crate::scene::Scene;
use crate::hitable_collection::HitableCollection;
use crate::cgmath::Vector3;
use crate::moving_sphere::MovingSphere;
use crate::sphere::Sphere;
use crate::camera::Camera;
use std::time::{Instant};
use crate::lambertian::Lambertian;
use crate::metal::Metal;
use crate::dielectric::Dielectric;
use std::sync::Arc;
use crate::cgmath::InnerSpace;
use crate::render_engine_multithread::RenderEngineMultithread;
use crate::hitable::Hitable;
use rand::Rng;
use crate::trait_output::Output;
use sdl2::log::Category::Render;
use crate::render_information::RenderInformation;
use crate::bvh_node::BVHNode;

fn random_scene(scene: &mut Vec<Arc<dyn Hitable>>) {
    let n = 500;
    scene.push(Arc::from(Sphere::new(Vector3::new(0.0, -1000.0, 0.0), 1000.0, Box::from(Lambertian::new(Vector3::new(0.5, 0.5, 0.5), 0.0)))));
    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = Vector3::new(a as f64+0.9*rng.gen::<f64>(), 0.2, b as f64+0.9*rng.gen::<f64>());
            if (center - Vector3::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                if choose_mat < 0.3 {
                    scene.push(Arc::from(MovingSphere::new(center, center + Vector3::new(0.0, rng.gen::<f64>() * 0.5, 0.0), 0.0, 1.0,0.2, Box::from(Metal::new(Vector3::new(0.9, 0.9, 0.9), 0.1 * rng.gen::<f64>())))));
                } else if choose_mat < 0.75 {
                    scene.push(Arc::from(MovingSphere::new(center, center + Vector3::new(0.0, rng.gen::<f64>() * 0.5, 0.0), 0.0, 1.0, 0.2, Box::from(Lambertian::new(Vector3::new(rng.gen::<f64>()*rng.gen::<f64>(), rng.gen::<f64>()*rng.gen::<f64>(), rng.gen::<f64>()*rng.gen::<f64>()), 0.0)))));
                } else {
                    scene.push(Arc::from(MovingSphere::new(center, center + Vector3::new(0.0, rng.gen::<f64>() * 0.5, 0.0), 0.0, 1.0, 0.2, Box::from(Dielectric::new(1.5)))));
                }
            }
        }
    }
    scene.push(Arc::from(Sphere::new(Vector3::new(0.0, 1.0, 0.0), 1.0, Box::from(Dielectric::new(1.5)))));
    scene.push(Arc::from(Sphere::new(Vector3::new(-4.0, 1.0, 0.0), 1.0, Box::from(Lambertian::new(Vector3::new(1.0, 0.5, 0.8), 0.0)))));
    scene.push(Arc::from(Sphere::new(Vector3::new(4.0, 1.0, 0.0), 1.0, Box::from(Metal::new(Vector3::new(0.7, 0.8, 0.7), 0.0)))));
}

fn main() {
    let width = 640;
    let height = 480;
    let mut img: Box<dyn Output> = Box::from(sdl_preview::sdl_output::SDLOutput::new(width, height));
    //let mut img: Box<dyn Output> = Box::from(ImageOutput::new(width, height));
    let world = Arc::from(HitableCollection{
        list:vec![
            Arc::from(MovingSphere::new(Vector3::new(0.0,0.0,-1.0), Vector3::new(0.0,0.2,-1.0), 0.0, 1.5,0.5, Box::from(Lambertian::new(Vector3::new(0.2, 0.8, 0.1), 0.0)))),
            Arc::from(Sphere::new(Vector3::new(0.0,-100.5,-1.0), 100.0,Box::from(Lambertian::new(Vector3::new(0.1, 0.2, 0.5), 0.0)))),
            //Box::from(Sphere::new(Vector3::new(0.0,-100.5,-1.0), 100.0,Box::from(Metal::new(Vector3::new(1.0, 1.0, 1.0), 0.0)))),
            //Box::from(Sphere::new(Vector3::new(1.0, 0.0, -1.0), 0.5, Box::from(Metal::new(Vector3::new(0.8,0.6,0.2), 0.3)))),
            Arc::from(Sphere::new(Vector3::new(1.0,0.0,-1.0), 0.5, Box::from(Metal::new(Vector3::new(0.9, 0.89, 0.91), 0.01)))),
            Arc::from(Sphere::new(Vector3::new(-1.1,0.0,-1.0), 0.5, Box::from(Dielectric::new(1.517)))),
            Arc::from(Sphere::new(Vector3::new(-1.1,0.0,-1.0), -0.45, Box::from(Dielectric::new(1.517)))),
        ]
    });
    // let mut scene_hits = Vec::<Arc<dyn Hitable>>::new();
    // random_scene(&mut scene_hits);
    // let world = Arc::from(HitableCollection{
    //     list: scene_hits,
    // });
    // let len = scene_hits.len();
    // let world = Arc::from(BVHNode::new(scene_hits, 0, len-1, 0.0, 1.0));
    let start = Instant::now();
    // let lookfrom = Vector3::<f64>::new(15.0, 1.0, -3.0);
    // let lookat = Vector3::new(0.0, 0.5, 0.0);
    let lookfrom = Vector3::<f64>::new(0.0, 1.0, 1.0);
    let lookat = Vector3::new(0.0, 0.5, 0.0);
    let dist_to_focus = (lookfrom-lookat).magnitude();
    let aperture = 0.01;
    let samples = 100;
    RenderEngineMultithread::render(
        Box::from(
            Scene {
                width,
                height,
                world,
                camera: Camera::new(
                    lookfrom,
                    lookat,
                    Vector3::new(0.0, -1.0, 0.0),
                    //20.0,
                    40.0,
                    width as f64 / height as f64,
                    aperture,
                    dist_to_focus,
                    samples,
                    0.0,
                    1.0,
                ),
            }),
        &mut img,
        50,
        15,
        true,
    );
    let render_duration: u128 = start.elapsed().as_millis();
    println!("\nFinished render in: {}", render_duration);
    //img.remap_pixel_range(1.0);
    img.save("test.png", RenderInformation {
        duration: render_duration as u32,
        avg_per_pixel:  (render_duration / (width as u128 * height as u128)) as u32,
        samples: samples as u32,
    });
}