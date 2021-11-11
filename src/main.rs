#![allow(dead_code)]
mod camera;
mod hittable;
mod hittablelist;
mod image;
mod material;
mod ray;
mod shapes;
mod vectors;

use std::sync::{atomic::AtomicBool, Arc, Mutex};

use crate::{
    material::{Lambertian, Metal},
    vectors::*,
};
use anyhow::Result;
use camera::Camera;
use hittable::HitRecord;
use hittablelist::HittableList;
use indicatif::{ProgressBar, ProgressStyle};
use ray::Ray;
use shapes::sphere::Sphere;

fn main() -> Result<()> {
    // Image
    let image_width = 1000;
    let image_height = 1000;
    if (image_width % 2) != 0 {
        panic!("image width must be even");
    }
    if image_height % 2 != 0 {
        panic!("image height must be even");
    }
    let samples_per_pixel = 1;
    let max_depth = 50;
    // World
    let mut world = HittableList::new();
    let material_ground = Box::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Box::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let material_left = Box::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Box::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.3));
    let bar = Arc::new(Mutex::new(
        ProgressBar::new(image_width as u64 * image_height as u64).with_style(
            ProgressStyle::default_bar().template(
                "{bar:40} [{per_sec} pixels per second] [{elapsed_precise} elapsed] [{eta} left]",
            ),
        ),
    ));

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    // Camera
    let camera = Camera::new(image_width as f32 / image_height as f32);

    let now = std::time::Instant::now();
    let image = Arc::new(Mutex::new(image::Image::new(image_width, image_height)));
    let image_clone = image.clone();
    rayon::scope(|s| {
        for y in 0..image_height {
            let world = world.clone();
            let camera = camera.clone();
            let row = Arc::new(Mutex::new(vec![Color::new_all(0.0); image_width as usize]));
            let row_clone = row.clone();
            let bar_clone = bar.clone();
            let image_clone = image_clone.clone();

            s.spawn(move |_| {
                let mut row = row_clone.lock().unwrap();
                for x in 0..image_width {
                    let mut color = Color::new_all(0.0);
                    for _ in 0..samples_per_pixel {
                        let u = (x as f32 + rand::random::<f32>()) / (image_width - 1) as f32;
                        let v = (y as f32 + rand::random::<f32>()) / (image_height - 1) as f32;
                        let r = camera.get_ray(u, v);
                        color = color + ray_color(r, &world, max_depth);
                    }
                    row[x as usize] = color;
                    if x % 4 == 0 {
                        bar_clone.lock().unwrap().inc(4);
                    }
                }
                {
                    let mut img = image_clone.lock().unwrap();
                    for x in 0..image_width {
                        let color = row[x as usize];

                        img.set_pixel(x, y, color, samples_per_pixel);
                    }
                }
            });
        }
    });
    bar.lock().unwrap().finish();
    let elapsed = now.elapsed();
    println!("Took {}s", elapsed.as_secs_f64());
    {
        image
            .lock()
            .unwrap()
            .save("test.png")
            .expect("Failed to save image");
        println!("Saved image");
    }

    Ok(())
}
fn ray_color(r: Ray, world: &HittableList, depth: u32) -> Color {
    let mut rec = HitRecord::empty();
    if depth <= 0 {
        return Color::new_all(0.0);
    }

    if world.hit(r, 0.001, f32::MAX, &mut rec) {
        if let Some((scattered, attenuation)) = rec.mat.scatter(r, &rec) {
            return attenuation * ray_color(scattered, world, depth - 1);
        }
        return Color::new_all(0.0);
    }
    let unit_direction = r.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    return Vec3::new_all(1.0 - t) * Color::new(1.0, 1.0, 1.0)
        + Vec3::new_all(t) * Color::new(0.5, 0.7, 1.0);
}
fn hit_sphere(center: Point3, radius: f32, r: Ray) -> f32 {
    let oc = r.origin - center;
    let a = r.direction.length_squared();
    let half_b = oc.dot(r.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / a;
    }
}
