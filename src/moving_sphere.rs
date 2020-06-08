use cgmath::{Vector3, InnerSpace};
use crate::hitable::{Hitable, HitRecord};
use crate::ray::Ray;
use crate::material::Material;

pub struct MovingSphere {
    center1: Vector3<f64>,
    center0: Vector3<f64>,
    time0: f64,
    time1: f64,
    radius: f64,
    material: Box<dyn Material>,
}

impl MovingSphere {
    pub fn new(center0: Vector3<f64>, center1: Vector3<f64>, t0: f64, t1: f64, radius: f64, material: Box<dyn Material>) -> MovingSphere {
        return MovingSphere {
            center0,
            center1,
            time0: t0,
            time1: t1,
            radius,
            material,
        };
    }
    fn center(&self, time: f64) -> Vector3<f64>{
        return self.center0 + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0);
    }
}

impl Hitable for MovingSphere  {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin - self.center(r.time);
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
                rec.normal = (rec.position - self.center(r.time)) / self.radius;
                return true;
            }
            temp = (-b + (b * b - a * c).sqrt())/a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.position = r.point_at_parameter(rec.t);
                rec.normal = (rec.position - self.center(r.time)) / self.radius;
                return true;
            }
        }
        return false;
    }

    fn duplicate(&self) -> Box<dyn Hitable> {
        return Box::from(MovingSphere{
            center0: self.center0,
            center1: self.center1,
            time0: self.time0,
            time1: self.time1,
            radius: self.radius,
            material: self.material.duplicate(),
        });
    }
}

impl std::clone::Clone for MovingSphere {
    fn clone(&self) -> Self {
        return MovingSphere {
            center0: self.center0,
            center1: self.center1,
            time0: self.time0,
            time1: self.time1,
            radius: self.radius,
            material: self.material.duplicate(),
        };
    }
}