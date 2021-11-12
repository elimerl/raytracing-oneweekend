#![allow(dead_code)]
mod camera;
mod hittable;
mod hittablelist;
mod image;
mod material;
mod ray;
mod shapes;
mod vectors;

use std::{
    fs::read,
    sync::{Arc, Mutex},
};

use crate::vectors::*;
use anyhow::Result;
use camera::Camera;
use hittable::HitRecord;
use hittablelist::HittableList;
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use ray::Ray;
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
    /// World file
    #[structopt(long, parse(from_os_str))]
    world: PathBuf,
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
    let block_width = image_width / highest_power_of_2(image_width);
    let block_height = image_height / highest_power_of_2(image_height);

    // World
    let world = serde_yaml::from_slice::<HittableList>(&read(opt.world)?)?;

    // Camera
    let camera = Camera::new(
        world.camera_pos,
        world.camera_lookat,
        Vec3::new(0.0, 1.0, 0.0),
        world.camera_fov,
        image_width as f32 / image_height as f32,
    );
    println!(
        r"Rendering to file {} at resolution {}x{} with {} samples and max recurse {}
With {}x{} blocks",
        opt.output.to_str().unwrap(),
        image_width,
        image_height,
        samples_per_pixel,
        max_depth,
        block_width,
        block_height
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

    rayon::scope(|s| {
        let world_clone = Arc::new(world);
        for block_x in 0..(image_width / block_width) {
            for block_y in 0..(image_height / block_height) {
                let image_clone = image_clone.clone();
                let bar_clone = bar.clone();
                let world_clone = world_clone.clone();
                s.spawn(move |_| {
                    for x in block_x * block_width..(block_x + 1) * block_width {
                        for y in block_y * block_height..(block_y + 1) * block_height {
                            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
                            for _s in 0..samples_per_pixel {
                                let u = (x as f32 + rand::thread_rng().gen::<f32>())
                                    / image_width as f32;
                                let v = (y as f32 + rand::thread_rng().gen::<f32>())
                                    / image_height as f32;
                                let r = camera.get_ray(u, v);
                                pixel_color = pixel_color + ray_color(r, &world_clone, max_depth);
                            }

                            image_clone.lock().unwrap().set_pixel(
                                x,
                                y,
                                pixel_color,
                                samples_per_pixel,
                            );
                        }
                        bar_clone.lock().unwrap().inc(16);
                    }
                });
            }
        }
    });
    // Finish
    bar.lock().unwrap().finish();
    let elapsed = now.elapsed();
    println!("Took {:.2}s", elapsed.as_secs_f64());
    {
        image
            .lock()
            .unwrap()
            .save(opt.output.to_str().unwrap())
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

fn highest_power_of_2(n: u32) -> u32 {
    return n & (!(n - 1));
}
