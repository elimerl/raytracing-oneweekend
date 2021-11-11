use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Sub},
};
#[derive(Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Debug for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "vec3({}, {}, {})", self.x, self.y, self.z)
    }
}
impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }
    pub fn new_all(value: f32) -> Vec3 {
        Vec3 {
            x: value,
            y: value,
            z: value,
        }
    }
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn normalize(&self) -> Vec3 {
        let mut cloned = *self;
        let length = cloned.length();
        cloned.x /= length;
        cloned.y /= length;
        cloned.z /= length;
        cloned
    }
    pub fn dot(&self, other: Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}
pub type Point3 = Vec3;
pub type Color = Vec3;
impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}
impl Div for Vec3 {
    type Output = Vec3;
    fn div(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}
#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;
    #[test]
    fn test_vector_new() {
        let v = super::Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_vector_length() {
        let v = super::Vec3::new(1.0, 2.0, 3.0);
        assert!(approx_eq!(f32, v.length(), 3.741657, ulps = 4));
    }

    #[test]
    fn test_vector_normalize() {
        let v = super::Vec3::new(1.0, 2.0, 3.0);
        let v_normalized = v.normalize();
        assert!(approx_eq!(f32, v_normalized.length(), 1.0, ulps = 4));
    }
    #[test]
    fn test_vector_dot() {
        let v1 = super::Vec3::new(1.0, 2.0, 3.0);
        let v2 = super::Vec3::new(2.0, 3.0, 4.0);
        assert!(approx_eq!(f32, v1.dot(v2), 20.0, ulps = 4));
    }
    #[test]
    fn test_vector_cross() {
        let v1 = super::Vec3::new(1.0, 2.0, 3.0);
        let v2 = super::Vec3::new(2.0, 3.0, 4.0);
        let v3 = v1.cross(v2);
        assert!(approx_eq!(f32, v3.x, -1.0, ulps = 4));
        assert!(approx_eq!(f32, v3.y, 2.0, ulps = 4));
        assert!(approx_eq!(f32, v3.z, -1.0, ulps = 4));
    }
}
