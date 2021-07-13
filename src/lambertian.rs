use cgmath::Vector3;
use crate::ray::Ray;
use crate::hitable::HitRecord;
use crate::material::Material;
use crate::render_engine_multithread::RenderEngineMultithread;
use crate::texture::Texture;
use crate::solid_color_texture::SolidColorTexture;
use dyn_clone::private::sync::Arc;

#[derive(Clone)]
pub struct Lambertian {
    albedo: Arc<dyn Texture>,
    emission: f64,
}

impl Material for Lambertian {
    fn scatter (&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vector3<f64>, scattered: &mut Ray) -> bool {
        let target = rec.position + rec.normal + RenderEngineMultithread::random_in_unit_sphere();
        /* *scattered = Ray {
            origin: rec.position,
            direction: target - rec.position,
        };*/
        *scattered = Ray::newTime(rec.position, target - rec.position, r_in.time);
        *attenuation = self.albedo.value(rec.u, rec.v, rec.position);
        //*emissive = false;
        /*if self.emission > 0.1 {
            *emissive = true;
            *emission = self.albedo * self.emission;
        }*/
        return true;
        /*
         * TODO Implement probability p of reflection
         * albedo/p
         */
    }

    fn duplicate(&self) -> Box<dyn Material> {
        return Box::from(Lambertian {
            albedo: self.albedo.clone(),
            emission: self.emission,
        });
    }
}

impl Lambertian {
    pub fn new_color (albedo: Vector3<f64>, emission: f64) -> Lambertian {
        return Lambertian {
            albedo: Arc::from(SolidColorTexture{ color: albedo }),
            emission,
        }
    }
    pub fn new_texture(albedo: Arc<dyn Texture>, emission: f64) -> Lambertian {
        return Lambertian {
            albedo,
            emission,
        };
    }
}