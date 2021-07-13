use crate::ray::Ray;
use crate::hitable::HitRecord;
use cgmath::Vector3;

use dyn_clone::DynClone;

pub trait Material: DynClone + Send + Sync {
    fn scatter (&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vector3<f64>, scattered: &mut Ray) -> bool;
    fn duplicate(&self) -> Box<dyn Material>;
}

dyn_clone::clone_trait_object!(Material);