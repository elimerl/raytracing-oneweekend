use crate::vectors::{Point3, Vec3};
#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }
    pub fn at(&self, t: f32) -> Point3 {
        self.origin + self.direction * Vec3::new_all(t)
    }
}
