use crate::hitable::{Hitable, HitRecord};
use crate::AABB::AABB;
use crate::ray::Ray;
use rand::Rng;
use cgmath::{Vector3, Zero};
use dyn_clone::private::cmp::Ordering;
use dyn_clone::private::sync::Arc;

pub struct BVHNode {
    left: Arc<dyn Hitable>,
    right: Arc<dyn Hitable>,
    aabb: AABB,
}

impl BVHNode {
    fn box_compare(a: &Arc<dyn Hitable>, b: &Arc<dyn Hitable>, axis: usize) -> bool {
        let mut box_a = AABB::new(Vector3::<f64>::zero(), Vector3::<f64>::zero());
        let mut box_b = AABB::new(Vector3::<f64>::zero(), Vector3::<f64>::zero());
        if !a.bounding_box(0.0,0.0, &mut box_a) || !b.bounding_box(0.0,0.0, &mut box_b) {
            eprintln!("No bounding box in bvh_node constructor.");
        }
        box_a.minimum[axis] < box_b.minimum[axis]
    }

    fn box_x_compare(a: &Arc<dyn Hitable>, b: &Arc<dyn Hitable>) -> bool {
        BVHNode::box_compare(a,b,0)
    }

    fn box_y_compare(a: &Arc<dyn Hitable>, b: &Arc<dyn Hitable>) -> bool {
        BVHNode::box_compare(a,b,1)
    }

    fn box_z_compare(a: &Arc<dyn Hitable>, b: &Arc<dyn Hitable>) -> bool {
        BVHNode::box_compare(a,b,2)
    }

    pub fn new(src_objects: Vec<Arc<dyn Hitable>>, start: usize, end: usize, time0: f64, time1: f64) -> BVHNode {
        let mut objects = src_objects.to_vec();
        let axis = rand::thread_rng().gen_range(0,3);
        let comparator = if axis == 0 {BVHNode::box_x_compare} else { if axis == 1 {BVHNode::box_y_compare} else {BVHNode::box_z_compare}};
        let object_span = end - start;
        let left: Arc<dyn Hitable>;
        let right: Arc<dyn Hitable>;
        if object_span == 1 {
            left = objects[start].duplicate();
            right = objects[start].duplicate();
        } else if object_span == 2 {
            println!("{}", start);
            if comparator(&objects[start], &objects[start + 1]) {
                left = objects[start].duplicate();
                right = objects[start + 1].duplicate();
            } else {
                left = objects[start+1].duplicate();
                right = objects[start].duplicate();
            }
        } else {
            objects.sort_by(|a,b| if comparator(a,b) {Ordering::Greater} else {Ordering::Less});
            let mid = start + object_span/2;
            left = Arc::from(BVHNode::new(objects[0..objects.len()].to_vec(), start, mid, time0 , time1));
            right = Arc::from(BVHNode::new(objects[0..objects.len()].to_vec(), mid, end, time0, time1));
        }
        let mut box_left = AABB::new(Vector3::<f64>::zero(), Vector3::<f64>::zero());
        let mut box_right = AABB::new(Vector3::<f64>::zero(), Vector3::<f64>::zero());
        if !left.bounding_box(time0, time1, &mut box_left) || !right.bounding_box(time0, time1, &mut box_right) {
            eprintln!("No bounding box in bvh_node constructor.");
        }
        BVHNode {
            left,
            right,
            aabb: AABB::surrounding_box(&box_left, &box_right),
        }
    }
}

impl Hitable for BVHNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if !self.aabb.hit(r, t_min, t_max) {
            return false;
        }
        let hit_left = self.left.hit(r, t_min, t_max, rec);
        let hit_right = self.right.hit(r, t_min, if hit_left {rec.t} else {t_max}, rec);
        hit_left || hit_right
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        output_box.maximum = self.aabb.maximum;
        output_box.minimum = self.aabb.minimum;
        true
    }

    fn duplicate(&self) -> Arc<dyn Hitable> {
        Arc::from(BVHNode {
            left: self.left.duplicate(),
            right: self.right.duplicate(),
            aabb: self.aabb,
        })
    }
}