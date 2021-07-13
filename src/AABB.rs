use cgmath::Vector3;
use crate::ray::Ray;
use std::cmp::{min, max};

#[derive(Copy, Clone)]
pub struct AABB {
    pub minimum: Vector3<f64>,
    pub maximum: Vector3<f64>,
}

impl AABB {
    pub fn new(minimum: Vector3<f64>, maximum: Vector3<f64>) -> AABB {
        return AABB {
            minimum,
            maximum,
        };
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        // for a in 0..3 {
        //     let t0 = min(
        //         (self.minimum[a] - r.origin[a]) / r.direction[a],
        //         (self.maximum[a] - r.origin[a]) / r.direction[a],
        //     );
        //     let t1 = min(
        //         (self.minimum[a] - r.origin[a]) / r.direction[a],
        //         (self.maximum[a] - r.origin[a]) / r.direction[a],
        //     );
        //     let true_t_min = max(t0, t_min);
        //     let true_t_max = min(t1, t_max);
        //     if true_t_max <= true_t_min {
        //         false
        //     }
        // }
        // true
        for a in 0..3 {
            let inv_d = 1.0 / r.direction[a];
            let mut t0 = (self.minimum[a] - r.origin[a]) * inv_d;
            let mut t1 = (self.maximum[a] - r.origin[a]) * inv_d;
            if inv_d < 0.0 {
                let tmp = t0;
                t0 = t1;
                t1 = tmp;
            }
            let true_t_min = if t0 > t_min {t0} else {t_min};
            let true_t_max = if t1 < t_max {t1} else {t_max};
            if true_t_max <= true_t_min {
                return false;
            }
        }
        true
    }

    pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
        let small = Vector3::<f64>::new(
            box0.minimum.x.min(box1.minimum.x),
            box0.minimum.y.min(box1.minimum.y),
            box0.minimum.z.min(box1.minimum.z),
        );
        let big = Vector3::<f64>::new(
            box0.maximum.x.max(box1.maximum.x),
            box0.maximum.y.max(box1.maximum.y),
            box0.maximum.z.max(box1.maximum.z),
        );
        AABB::new(small, big)
    }
}