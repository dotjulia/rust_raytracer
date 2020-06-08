use cgmath::Vector3;
use std::ops::{Add, DivAssign, Mul};

#[derive(Clone, Copy)]
pub struct Color {
    // values are between 0 and 1
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Add<Color> for Color {
    type Output = Color;
    fn add(self, color: Color) -> Self::Output {
        Color {
            r: self.r + color.r,
            g: self.g + color.g,
            b: self.b + color.b,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        return Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        };
    }
}

impl Mul<Vector3<f64>> for Color {
    type Output = Color;
    fn mul(self, rhs: Vector3<f64>) -> Self::Output {
        return Color {
            r: self.r * rhs.x,
            g: self.g * rhs.y,
            b: self.b * rhs.z,
        }
    }
}

impl DivAssign<Color> for Color {
    fn div_assign(&mut self, rhs: Color) {
        self.r /= rhs.r;
        self.g /= rhs.g;
        self.b /= rhs.b;
    }
}

impl DivAssign<f64> for Color {
    fn div_assign(&mut self, rhs: f64) {
        self.r /= rhs;
        self.g /= rhs;
        self.b /= rhs;
    }
}

impl Color {
    pub fn from_vector3(vec: Vector3<f64>) -> Color {
        return Color {
            r: vec.x,
            g: vec.y,
            b: vec.z,
        };
    }
}