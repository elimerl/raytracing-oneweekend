use std::fmt::Debug;

use crate::{
    hittable::HitRecord,
    ray::Ray,
    vectors::{Color, Vec3},
};
pub trait Material: Debug + MaterialClone + Send + Sync {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}
pub trait MaterialClone {
    fn clone_box(&self) -> Box<dyn Material>;
}
impl<T> MaterialClone for T
where
    T: 'static + Material + Clone,
{
    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct Lambertian {
    pub albedo: Color,
}
impl Material for Lambertian {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction);
        let attenuation = self.albedo;
        return Some((scattered, attenuation));
    }
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
    pub fn empty() -> Self {
        Self {
            albedo: Color::new_all(0.0),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Metal {
    albedo: Color,
    fuzzy: f32,
}
impl Metal {
    pub fn new(color: Color, fuzzy: f32) -> Self {
        Self {
            albedo: color,
            fuzzy,
        }
    }
}
impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = (r_in.direction).normalize().reflect(rec.normal);
        let scattered = Ray::new(
            rec.p,
            reflected + (Vec3::new_all(self.fuzzy) * Vec3::random_unit_vector()),
        );
        let attenuation = self.albedo;
        if scattered.direction.dot(rec.normal) > 0.0 {
            return Some((scattered, attenuation));
        } else {
            return None;
        }
    }
}
