use cgmath::{Vector3, InnerSpace};
use crate::hitable::{Hitable, HitRecord};
use crate::ray::Ray;
use crate::material::Material;
use crate::AABB::AABB;
use dyn_clone::private::sync::Arc;
use crate::texture::{Texture, UVCoords};
use dyn_clone::private::f64::consts::PI;

pub struct Sphere {
    center: Vector3<f64>,
    radius: f64,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn get_sphere_uv(p: Vector3<f64>) -> UVCoords {
        let theta = (-p.y).acos();
        let phi = (-p.z).atan2(p.x) + PI;
        return UVCoords::new(phi / (2 * PI), theta / PI);
    }
    pub fn new(center: Vector3<f64>, radius: f64, material: Box<dyn Material>) -> Sphere {
        return Sphere {
            center,
            radius,
            material,
        };
    }
}

impl Hitable for Sphere  {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin - self.center;
        let a = r.direction.dot(r.direction);
        let b = oc.dot(r.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        rec.material = self.material.duplicate();
        if discriminant > 0.0 {
            let mut temp = (-b - (b * b - a * c).sqrt())/a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.position = r.point_at_parameter(rec.t);
                rec.normal = (rec.position - self.center) / self.radius;

                let uv = Sphere::get_sphere_uv(rec.normal);
                rec.u = uv.x;
                rec.v = uv.v;
                return true;
            }
            temp = (-b + (b * b - a * c).sqrt())/a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.position = r.point_at_parameter(rec.t);
                rec.normal = (rec.position - self.center) / self.radius;

                let uv = Sphere::get_sphere_uv(rec.normal);
                rec.u = uv.x;
                rec.v = uv.v;
                return true;
            }
        }
        return false;
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        output_box.minimum = self.center - Vector3::<f64>::new(self.radius, self.radius, self.radius);
        output_box.maximum = self.center + Vector3::<f64>::new(self.radius, self.radius, self.radius);
        true
    }

    fn duplicate(&self) -> Arc<dyn Hitable> {
        return Arc::from(Sphere{
            center: self.center,
            radius: self.radius,
            material: self.material.duplicate(),
        });
    }
}

impl std::clone::Clone for Sphere {
    fn clone(&self) -> Self {
        return Sphere {
            center: self.center,
            radius: self.radius,
            material: self.material.duplicate(),
        };
    }
}