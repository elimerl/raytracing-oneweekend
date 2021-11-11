use crate::{
    ray::Ray,
    vectors::{Point3, Vec3},
};
#[derive(Clone, Copy)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}
fn deg2rad(deg: f32) -> f32 {
    deg * std::f32::consts::PI / 180.0
}
impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f32,
        aspect_ratio: f32,
    ) -> Camera {
        let mut cam = Camera {
            origin: Point3::new_all(0.0),
            lower_left_corner: Point3::new_all(0.0),
            horizontal: Vec3::new_all(0.0),
            vertical: Vec3::new_all(0.0),
        };
        let theta = deg2rad(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let _focal_length = 1.0;

        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        cam.origin = lookfrom;
        cam.horizontal = Vec3::new_all(viewport_width) * u;
        cam.vertical = Vec3::new_all(viewport_height) * v;
        cam.lower_left_corner = cam.origin
            - cam.horizontal / Vec3::new_all(2.0)
            - cam.vertical / Vec3::new_all(2.0)
            - w;

        cam
    }
    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        return Ray::new(
            self.origin,
            self.lower_left_corner
                + Vec3::new_all(s) * self.horizontal
                + Vec3::new_all(t) * self.vertical
                - self.origin,
        );
    }
}
