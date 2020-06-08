use crate::material::Material;
use cgmath::{Vector3, InnerSpace};
use crate::ray::Ray;
use crate::hitable::HitRecord;
use crate::metal::Metal;
use image::error::LimitErrorKind::DimensionError;
use rand::Rng;

#[derive(Copy, Clone)]
pub struct Dielectric {
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Dielectric {
        return Dielectric {
            ref_idx,
        };
    }

    fn refract(v: &Vector3<f64>, n: &Vector3<f64>, ni_over_nt: f64, refracted: &mut Vector3<f64>) -> bool {
        let uv = v.normalize();
        let dt = uv.dot(*n);
        let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
        if discriminant > 0.0 {
            *refracted = (uv - n * dt) * ni_over_nt - n * discriminant.sqrt();
            return true;
        } else {
            return false;
        }
    }
    fn schlick(&self, cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0-self.ref_idx) / (1.0+ref_idx);
        r0 = r0 * r0;
        return r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0);
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vector3<f64>, scattered: &mut Ray) -> bool {
        //*emissive = false;
        let mut outward_normal = Vector3::new(0.0, 0.0, 0.0);
        let reflected = Metal::reflect(r_in.direction, rec.normal);
        let ni_over_nt: f64;
        *attenuation = Vector3::new(1.0, 1.0, 1.0);
        let mut refracted = Vector3::new(0.0, 0.0, 0.0);
        let mut reflect_prob: f64;
        let mut cosine: f64;
        if r_in.direction.dot(rec.normal) > 0.0 {
            outward_normal = -rec.normal;
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * r_in.direction.dot(rec.normal) / r_in.direction.magnitude();
        } else {
            outward_normal = rec.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = - r_in.direction.dot(rec.normal) / r_in.direction.magnitude();
        }

        if Dielectric::refract(&r_in.direction, &outward_normal, ni_over_nt, &mut refracted) {
            reflect_prob = self.schlick(cosine, self.ref_idx);
            // *scattered = Ray {
            //     origin: rec.position,
            //     direction: refracted,
            // };
        } else {
            reflect_prob = 1.0;
            *scattered = Ray {
                origin: rec.position,
                direction: reflected,
            };
            //return false;
        }
        let mut rng = rand::thread_rng();
        if rng.gen::<f64>() < reflect_prob {
            *scattered = Ray {
                origin: rec.position,
                direction: reflected,
            };
        } else {
            *scattered = Ray {
                origin: rec.position,
                direction: refracted,
            };
        }
        return true;
    }

    fn duplicate(&self) -> Box<dyn Material> {
        return Box::from(Dielectric {
            ref_idx: self.ref_idx,
        });
    }
}