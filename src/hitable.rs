use cgmath::Vector3;
use crate::ray::Ray;

use std::fmt;
use crate::lambertian::Lambertian;
use crate::material::Material;

pub struct HitRecord {
    pub t: f64,
    pub position: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub material: Box<dyn Material>,
}

impl fmt::Display for HitRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HitRecord [t: {}, position: Vector3[{}, {}, {}], normal: Vector3[{}, {}, {}]]", self.t, self.position.x, self.position.y, self.position.z, self.normal.x, self.normal.y, self.position.z)
    }
}

impl HitRecord {
    pub fn new_empty() -> HitRecord{
        return HitRecord{
            t: 0.0,
            position: Vector3::new(0.0,0.0,0.0),
            normal: Vector3::new(0.0,0.0,0.0),
            material: Box::from(Lambertian::new(Vector3::new(1.0, 1.0, 1.0), 0.0)),
        };
    }
}

pub trait Hitable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
    fn duplicate(&self) -> Box<dyn Hitable>;
}