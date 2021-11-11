#![allow(dead_code)]
mod camera;
mod hittable;
mod hittablelist;
mod image;
mod material;
mod ray;
mod shapes;
mod vectors;

use std::sync::{Arc, Mutex};

use crate::{
    material::{Lambertian, Metal},
    vectors::*,
};
use anyhow::Result;
use camera::Camera;
use hittable::HitRecord;
use hittablelist::HittableList;
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use ray::Ray;
use shapes::sphere::Sphere;
use std::path::PathBuf;
use structopt::StructOpt;
#[derive(Debug, StructOpt)]
#[structopt(
    name = "raytracer",
    about = "A raytracer based on Ray Tracing in One Weekend"
)]
struct Opt {
    /// Output file
    #[structopt(parse(from_os_str))]
    output: PathBuf,
    // /// World file
    // #[structopt(long, parse(from_os_str))]
    // world: PathBuf,
    /// Width of the image
    #[structopt(short, long, default_value = "800")]
    width: u32,
    /// Height of the image
    #[structopt(short, long, default_value = "450")]
    height: u32,

    /// Number of samples per pixel
    #[structopt(short, long, default_value = "100")]
    samples: u32,

    /// Number of threads to use [default: number of cores]
    #[structopt(short, long)]
    threads: Option<u32>,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    if let Some(threads) = opt.threads {
        rayon::ThreadPoolBuilder::default()
            .num_threads(threads as usize)
            .build_global()
            .unwrap();
    }
    // Image
    let image_width = opt.width;
    let image_height = opt.height;
    if (image_width % 2) != 0 {
        panic!("image width must be even");
    }
    if image_height % 2 != 0 {
        panic!("image height must be even");
    }
    let samples_per_pixel = opt.samples;
    let max_depth = 50;

    // World
    let world = random_scene();

    // Camera
    let camera = Camera::new(
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        image_width as f32 / image_height as f32,
    );
    println!(
        "Rendering to file {} at resolution {}x{} with {} samples and max recurse {}",
        opt.output.to_str().unwrap(),
        image_width,
        image_height,
        samples_per_pixel,
        max_depth
    );
    // Progress bar
    let bar = Arc::new(Mutex::new(
        ProgressBar::new(image_width as u64 * image_height as u64).with_style(
            ProgressStyle::default_bar().template(
                "{bar:40} [{per_sec} pixels per second] [{elapsed_precise} elapsed] [{eta_precise} left]",
            ),
        ),
    ));
    // Render
    let now = std::time::Instant::now();
    let image = Arc::new(Mutex::new(image::Image::new(image_width, image_height)));
    let image_clone = image.clone();
    let bands = 8;
    rayon::scope(|s| {
        for band in 0..bands {
            let world = world.clone();
            let camera = camera.clone();
            let bar_clone = bar.clone();
            let image_clone = image_clone.clone();

            s.spawn(move |_| {
                let mut data =
                    vec![Color::new_all(0.0); ((image_height / bands) * image_width) as usize];
                for y in (band * (image_height / bands))..((band + 1) * (image_height / bands)) {
                    for x in 0..image_width {
                        let mut color = Color::new_all(0.0);
                        for _ in 0..samples_per_pixel {
                            let u = (x as f32 + rand::random::<f32>()) / (image_width - 1) as f32;
                            let v = (y as f32 + rand::random::<f32>()) / (image_height - 1) as f32;
                            let r = camera.get_ray(u, v);
                            color = color + ray_color(r, &world, max_depth);
                        }
                        data[(y - (band * (image_height / bands))) as usize
                            * image_width as usize
                            + x as usize] = color;
                        if x % 4 == 0 {
                            bar_clone.lock().unwrap().inc(4);
                        }
                    }
                }
                {
                    let mut img = image_clone.lock().unwrap();
                    for y in (band * (image_height / bands))..((band + 1) * (image_height / bands))
                    {
                        for x in 0..image_width {
                            let color = data[(y - (band * (image_height / bands))) as usize
                                * image_width as usize
                                + x as usize];

                            img.set_pixel(x, y, color, samples_per_pixel);
                        }
                    }
                }
            });
        }
    });
    // Finish
    bar.lock().unwrap().finish();
    let elapsed = now.elapsed();
    println!(
        "Took {}s at {} pixels per second",
        elapsed.as_secs_f64(),
        bar.lock().unwrap().per_sec()
    );
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
fn random_scene() -> HittableList {
    let mut world = HittableList::new();
    let ground_material = Box::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f32 = rand::random();
            let center = Point3::new(
                (a as f32) + 0.9 * rand::random::<f32>(),
                0.2,
                (b as f32) + 0.9 * rand::random::<f32>(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Lambertian::new(albedo);
                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        Box::new(sphere_material),
                    )));
                } else {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = rand::thread_rng().gen_range(0.0..0.5);
                    let sphere_material = Metal::new(albedo, fuzz);
                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        Box::new(sphere_material),
                    )));
                }
            }
        }
    }
    let material1 = Box::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Box::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Box::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}
