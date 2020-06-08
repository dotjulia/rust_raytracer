use crate::hitable::{Hitable, HitRecord};
use crate::ray::Ray;

pub struct HitableCollection {
    pub list: Vec<Box<dyn Hitable>>,
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

    fn duplicate(&self) -> Box<dyn Hitable> {
        let mut list = Vec::<Box<dyn Hitable>>::new();
        for h in self.list.as_slice() {
            list.push(h.duplicate());
        }
        return Box::from(HitableCollection {
            list,
        });
    }
}