use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
};

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
    fn hit(&self, r: Ray, t_min: f32, t_max: f32, _rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for object in &self.objects {
            if let Some(rec) = object.hit(r, t_min, closest_so_far) {
                hit_anything = true;
                closest_so_far = rec.t;
            }
        }
        hit_anything
    }
}
