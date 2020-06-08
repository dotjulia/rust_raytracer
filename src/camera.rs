use crate::ray::Ray;
use cgmath::{Vector3, InnerSpace};
use rand::Rng;

pub struct Camera {
    origin: Vector3<f64>,
    lower_left_corner: Vector3<f64>,
    horizontal: Vector3<f64>,
    vertical: Vector3<f64>,
    pub antialiasing_iterations: i32,
    lens_radius: f64,
    u: Vector3<f64>,
    v: Vector3<f64>,
    w: Vector3<f64>,
    time0: f64,
    time1: f64,
}

impl Camera {

    pub fn random_in_unit_disk() -> Vector3<f64>{
        let mut rng = rand::thread_rng();
        let mut p = Vector3::new(rng.gen::<f64>(),rng.gen::<f64>(),0.0) * 2.0 - Vector3::new(1.0, 1.0, 0.0);
        while p.dot(p) >= 1.0 {
            p = Vector3::new(rng.gen::<f64>(),rng.gen::<f64>(),0.0) * 2.0 - Vector3::new(1.0, 1.0, 0.0);
        }
        return p;
    }

    pub fn new(lookfrom: Vector3<f64>, lookat:Vector3<f64>,vup: Vector3<f64>, vfov: f64, aspect: f64, aperture: f64, focus_dist: f64, samples: i32, t0: f64, t1: f64) -> Camera {
        let lens_radius = aperture / 2.0;
        let theta = vfov * std::f64::consts::PI / 180.0;
        let half_height = (theta/2.0).tan();
        let half_width = aspect * half_height;
        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);
        return Camera {
            //lower_left_corner: Vector3::new(-half_width, -half_height, -1.0),
            lower_left_corner: lookfrom - u * focus_dist * half_width  - v * focus_dist * half_height - focus_dist * w,
            //horizontal: Vector3::new(2.0 * half_height, 0.0, 0.0),
            horizontal: u * half_width * focus_dist * 2.0,
            //vertical: Vector3::new(0.0, 2.0 * half_height, 0.0),
            vertical: v * half_height * focus_dist * 2.0,
            origin: lookfrom,
            antialiasing_iterations: samples,
            lens_radius,
            u,
            v,
            w,
            time0: t0,
            time1: t1,
        };
    }

    pub fn duplicate(&self) -> Camera {
        return Camera{
            lower_left_corner: self.lower_left_corner,
            horizontal: self.horizontal,
            vertical: self.vertical,
            origin: self.origin,
            antialiasing_iterations: self.antialiasing_iterations,
            lens_radius: self.lens_radius,
            u: self.u,
            v: self.v,
            w: self.w,
            time0: self.time0,
            time1: self.time1,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = Camera::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        let mut rng = rand::thread_rng();
        let time = self.time0 + rng.gen::<f64>() * (self.time1 - self.time0);
        return Ray::newTime(self.origin + offset, self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset, time);
        /*return Ray {
            origin: self.origin + offset,
            //origin: self.origin,
            //direction: self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
            direction: self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
        }; // Blurry background no work :(*/
    }
}