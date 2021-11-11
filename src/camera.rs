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
impl Camera {
    pub fn new(aspect_ratio: f32) -> Camera {
        let mut cam = Camera {
            origin: Point3::new_all(0.0),
            lower_left_corner: Point3::new_all(0.0),
            horizontal: Vec3::new_all(0.0),
            vertical: Vec3::new_all(0.0),
        };
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        cam.origin = Point3::new(0.0, 0.0, 0.0);
        cam.horizontal = Point3::new(viewport_width, 0.0, 0.0);
        cam.vertical = Point3::new(0.0, viewport_height, 0.0);
        cam.lower_left_corner = cam.origin
            - cam.horizontal / Vec3::new_all(2.0)
            - cam.vertical / Vec3::new_all(2.0)
            - Point3::new(0.0, 0.0, focal_length);

        cam
    }
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        return Ray::new(
            self.origin,
            self.lower_left_corner
                + Vec3::new_all(u) * self.horizontal
                + Vec3::new_all(v) * self.vertical
                - self.origin,
        );
    }
}
