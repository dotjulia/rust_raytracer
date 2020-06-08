use cgmath::{Vector3, InnerSpace};
use crate::material::Material;
use crate::ray::Ray;
use crate::hitable::HitRecord;
use crate::render_engine::RenderEngine;

#[derive(Copy, Clone)]
pub struct Metal {
    albedo: Vector3<f64>,
    fuzz: f64,
}

impl Metal {
    pub fn reflect(v: Vector3<f64>, n: Vector3<f64>) -> Vector3<f64> {
        return v - n * v.dot(n) * 2.0;
    }
    pub fn new(albedo: Vector3<f64>, fuzz: f64) -> Metal {
        return Metal {
            albedo,
            fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vector3<f64>, scattered: &mut Ray) -> bool {
        let reflected = Metal::reflect(r_in.direction.normalize(), rec.normal);
        *scattered = Ray{origin: rec.position, direction: reflected + RenderEngine::random_in_unit_sphere() * self.fuzz};
        *attenuation = self.albedo;
        return scattered.direction.dot(rec.normal) > 0.0;
    }

    fn duplicate(&self) -> Box<dyn Material> {
        return Box::from(Metal {
            albedo: self.albedo,
            fuzz: self.fuzz,
        });
    }
}