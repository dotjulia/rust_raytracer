extern crate cgmath;

use cgmath::Vector3;
use std::fmt;

pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction: Vector3<f64>,
    pub time: f64,
}

impl fmt::Display for Ray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ray [origin: Vector3[{}, {}, {}], direction: Vector3[{}, {}, {}]]", self.origin.x, self.origin.y, self.origin.z, self.direction.x, self.direction.y, self.direction.z)
    }
}

impl Ray {
    pub fn point_at_parameter(&self, t: f64) -> Vector3<f64> {
        return self.origin + self.direction * t;
    }

    pub fn newTime(origin: Vector3<f64>, direction: Vector3<f64>, time: f64) -> Ray {
        return Ray {
            origin,
            direction,
            time,
        }
    }

    pub fn new(origin: Vector3<f64>, direction: Vector3<f64>) -> Ray {
        return Ray {
            origin,
            direction,
            time: 0.0,
        }
    }

    pub fn new_empty() -> Ray {
        return Ray {
            origin: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(0.0, 0.0, 0.0),
            time: 0.0,
        }
    }
}