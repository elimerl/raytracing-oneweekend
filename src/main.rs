#![allow(dead_code)]
mod hittable;
mod hittablelist;
mod image;
mod ray;
mod shapes;
mod vectors;
use std::{sync::mpsc::channel, thread};

use crate::vectors::*;
use anyhow::Result;
use minifb::{Key, Window, WindowOptions};
use ray::Ray;
fn main() -> Result<()> {
    let aspect_ratio = 16.0 / 9.0 as f32;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as u32;
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let mut buffer: Vec<u32> = vec![0; (image_width * image_height) as usize];
    let mut window = Window::new(
        "Image - ESC to exit",
        image_width as usize,
        image_height as usize,
        WindowOptions {
            scale: minifb::Scale::X4,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let (sender, receiver) = channel();
    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    thread::spawn(move || {
        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner = origin
            - horizontal / Vec3::new_all(2.0)
            - vertical / Vec3::new_all(2.0)
            - Vec3::new(0.0, 0.0, focal_length);

        let mut image = image::Image::new(image_width, image_height);
        for y in 0..image_height {
            for x in 0..image_width {
                let u = x as f32 / (image_width - 1) as f32;
                let v = y as f32 / (image_height - 1) as f32;

                let r = Ray::new(
                    origin,
                    lower_left_corner + Vec3::new_all(u) * horizontal + Vec3::new_all(v) * vertical
                        - origin,
                );
                image.set_pixel(x, y, ray_color(r));
            }
            if y % (image_height / 4) == 0 {
                sender.send(image.image.clone()).unwrap();
            }
        }
        image.save("test.png").expect("Failed to save image");
    });
    // let progess_copy2 = progress.clone();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        if let Ok(image) = receiver.recv() {
            for x in 0..image_width {
                for y in 0..image_height {
                    let pixel = image.get_pixel(x, y);
                    let r = pixel[0] as u32;
                    let g = pixel[1] as u32;
                    let b = pixel[2] as u32;

                    // buffer[(y * image_width + x) as usize] = pixel.into();
                    buffer[(y * image_width + x) as usize] = (r << 16) + (g << 8) + b;
                }
            }
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, image_width as usize, image_height as usize)
            .unwrap();
    }

    Ok(())
}
fn ray_color(r: Ray) -> Color {
    let t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let n = (r.at(t) - Vec3::new(0.0, 0.0, -1.0)).normalize();
        return Color::new_all(0.5) * Color::new(n.x + 1.0, n.y + 1.0, n.z + 1.0);
    }

    let unit_direction = r.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    return Color::new_all(1.0 - t) * Color::new(1.0, 1.0, 1.0)
        + Color::new_all(t) * Color::new(0.5, 0.7, 1.0);
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
