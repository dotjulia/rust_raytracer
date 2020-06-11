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

fn random_scene(scene: &mut Vec<Box<dyn Hitable>>) {
    let n = 500;
    scene.push(Box::from(Sphere::new(Vector3::new(0.0, -1000.0, 0.0), 1000.0, Box::from(Lambertian::new(Vector3::new(0.5, 0.5, 0.5), 0.0)))));
    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = Vector3::new(a as f64+0.9*rng.gen::<f64>(), 0.2, b as f64+0.9*rng.gen::<f64>());
            if (center - Vector3::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                if choose_mat < 0.3 {
                    scene.push(Box::from(MovingSphere::new(center, center + Vector3::new(0.0, rng.gen::<f64>() * 0.5, 0.0), 0.0, 1.0,0.2, Box::from(Metal::new(Vector3::new(0.9, 0.9, 0.9), 0.1 * rng.gen::<f64>())))));
                } else if choose_mat < 0.75 {
                    scene.push(Box::from(MovingSphere::new(center, center + Vector3::new(0.0, rng.gen::<f64>() * 0.5, 0.0), 0.0, 1.0, 0.2, Box::from(Lambertian::new(Vector3::new(rng.gen::<f64>()*rng.gen::<f64>(), rng.gen::<f64>()*rng.gen::<f64>(), rng.gen::<f64>()*rng.gen::<f64>()), 0.0)))));
                } else {
                    scene.push(Box::from(MovingSphere::new(center, center + Vector3::new(0.0, rng.gen::<f64>() * 0.5, 0.0), 0.0, 1.0, 0.2, Box::from(Dielectric::new(1.5)))));
                }
            }
        }
    }
    scene.push(Box::from(Sphere::new(Vector3::new(0.0, 1.0, 0.0), 1.0, Box::from(Dielectric::new(1.5)))));
    scene.push(Box::from(Sphere::new(Vector3::new(-4.0, 1.0, 0.0), 1.0, Box::from(Lambertian::new(Vector3::new(1.0, 0.5, 0.8), 0.0)))));
    scene.push(Box::from(Sphere::new(Vector3::new(4.0, 1.0, 0.0), 1.0, Box::from(Metal::new(Vector3::new(0.7, 0.8, 0.7), 0.0)))));
}

fn main() {
    let width = 400;
    let height = 200;
    let mut img: Box<dyn Output> = Box::from(ImageOutput::new(width, height));
    // let world = Box::from(HitableCollection{
    //     list:vec![
    //         Box::from(Sphere::new(Vector3::new(0.0,0.0,-1.0), 0.5, Box::from(Lambertian::new(Vector3::new(0.2, 0.8, 0.1), 0.0)))),
    //         Box::from(Sphere::new(Vector3::new(0.0,-100.5,-1.0), 100.0,Box::from(Lambertian::new(Vector3::new(0.1, 0.2, 0.5), 0.0)))),
    //         //Box::from(Sphere::new(Vector3::new(0.0,-100.5,-1.0), 100.0,Box::from(Metal::new(Vector3::new(1.0, 1.0, 1.0), 0.0)))),
    //         //Box::from(Sphere::new(Vector3::new(1.0, 0.0, -1.0), 0.5, Box::from(Metal::new(Vector3::new(0.8,0.6,0.2), 0.3)))),
    //         Box::from(Sphere::new(Vector3::new(1.0,0.0,-1.0), 0.5, Box::from(Metal::new(Vector3::new(0.9, 0.89, 0.91), 0.01)))),
    //         Box::from(Sphere::new(Vector3::new(-1.1,0.0,-1.0), 0.5, Box::from(Dielectric::new(1.517)))),
    //         Box::from(Sphere::new(Vector3::new(-1.1,0.0,-1.0), -0.45, Box::from(Dielectric::new(1.517)))),
    //     ]
    // });
    let mut sceneHits = Vec::<Box<dyn Hitable>>::new();
    random_scene(&mut sceneHits);
    let world = Box::from(HitableCollection{
        list: sceneHits,
    });
    let start = Instant::now();
    let lookfrom = Vector3::<f64>::new(15.0, 1.0, -3.0);
    let lookat = Vector3::new(0.0, 0.5, 0.0);
    let dist_to_focus = (lookfrom-lookat).magnitude();
    let aperture = 0.01;
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
                    20.0,
                    width as f64 / height as f64,
                    aperture,
                    dist_to_focus,
                    40,
                    0.0,
                    1.0,
                ),
            }),
        &mut img,
        50,
        8,
    );
    println!("\nFinished render in: {}", start.elapsed().as_millis());
    //img.remap_pixel_range(1.0);
    img.save("test.png");
}