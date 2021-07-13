use crate::hitable::{Hitable, HitRecord};
use crate::ray::Ray;
use crate::AABB::AABB;
use cgmath::{Vector3, Zero};
use dyn_clone::private::sync::Arc;

pub struct HitableCollection {
    pub list: Vec<Arc<dyn Hitable>>,
}

impl Hitable for HitableCollection {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for hitable in &self.list {
            let mut temp_rec = HitRecord::new_empty();
            if hitable.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }
        return hit_anything;
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        if self.list.is_empty() {
            return false;
        }
        let mut temp_box: AABB = AABB::new(Vector3::<f64>::zero(), Vector3::<f64>::zero());
        let mut first_box = true;
        for object in &self.list {
            if !object.bounding_box(time0, time1, &mut temp_box) {
                return false;
            }
            let sur_box = AABB::surrounding_box(output_box, &temp_box);
            let ob = if first_box {&temp_box} else {&sur_box};
            output_box.minimum = ob.minimum;
            output_box.maximum = ob.maximum;
            first_box = false;
        }
        true
    }

    fn duplicate(&self) -> Arc<dyn Hitable> {
        let mut list = Vec::<Arc<dyn Hitable>>::new();
        for h in self.list.as_slice() {
            list.push(h.duplicate());
        }
        return Arc::from(HitableCollection {
            list,
        });
    }
}