use crate::{
    hittable::{HitRecord, Hittable},
    vectors::Vec3,
};

struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Hittable for Sphere {
    fn hit(
        &self,
        r: crate::ray::Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<crate::hittable::HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = (discriminant).sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let mut rec = HitRecord {
            t: root,
            p: r.at(root),
            normal: Vec3::new_all(0.0),
            front_face: false,
        };
        let outward_normal = (rec.p - self.center) / Vec3::new_all(self.radius);

        rec.set_face_normal(r, outward_normal);

        Some(rec)
    }
}
