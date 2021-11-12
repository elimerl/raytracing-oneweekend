use serde::{Deserialize, Serialize};

use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
    vectors::Vec3,
};
#[derive(Serialize, Deserialize, Debug)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
    pub camera_pos: Vec3,
    pub camera_lookat: Vec3,
    pub camera_fov: f32,
}
impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
            camera_lookat: Vec3::new(0.0, 0.0, 0.0),
            camera_pos: Vec3::new(0.0, 0.0, 0.0),
            camera_fov: 0.0,
        }
    }
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
    pub fn hit(&self, r: Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for object in &self.objects {
            if let Some(rec2) = object.hit(r, t_min, closest_so_far) {
                rec.p = rec2.p;
                rec.t = rec2.t;
                rec.normal = rec2.normal;
                rec.front_face = rec2.front_face;
                rec.mat = rec2.mat;
                hit_anything = true;
                closest_so_far = rec.t;
            }
        }
        hit_anything
    }
}
impl Clone for HittableList {
    fn clone(&self) -> Self {
        HittableList {
            objects: self.objects.iter().map(|x| x.clone_box()).collect(),
            camera_lookat: self.camera_lookat,
            camera_pos: self.camera_pos,
            camera_fov: self.camera_fov,
        }
    }
}
