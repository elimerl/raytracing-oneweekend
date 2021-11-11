use std::fmt::Debug;

use crate::{
    material::{Lambertian, Material},
    ray::Ray,
    vectors::{Point3, Vec3},
};
#[derive(Debug)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub mat: Box<dyn Material>,
}
impl HitRecord {
    pub fn empty() -> HitRecord {
        HitRecord {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
            mat: Box::new(Lambertian::empty()),
        }
    }
    #[inline(always)]
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        self.front_face = r.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}
pub trait Hittable: Send + Sync + HittableClone {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
pub trait HittableClone {
    fn clone_box(&self) -> Box<dyn Hittable>;
}
impl<T> HittableClone for T
where
    T: 'static + Hittable + Clone,
{
    fn clone_box(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}
