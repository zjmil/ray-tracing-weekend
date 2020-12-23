mod aabb;
mod camera;
mod hittable;
mod material;
mod moving_sphere;
mod perlin;
mod ray;
mod sphere;
mod texture;
mod util;
mod vec3;

use camera::Camera;
use hittable::{Hittable, SharedHittable};
use material::*;
use moving_sphere::MovingSphere;
use ray::Ray;
use rayon::prelude::*;
use sphere::Sphere;
use std::iter;
use std::sync::Arc;
use texture::*;
use util::*;
use vec3::*;

use std::f64::INFINITY;

fn write_color(color: &Color, samples_per_pixel: i32) {
    // Divide the color by the number of samples and gamma-correct for gamma=2.0
    let scale = 1.0 / (samples_per_pixel as f64);
    let sr = (scale * color.x).sqrt();
    let sg = (scale * color.y).sqrt();
    let sb = (scale * color.z).sqrt();

    let r = (256.0 * clamp(sr, 0.0, 0.999)) as i32;
    let g = (256.0 * clamp(sg, 0.0, 0.999)) as i32;
    let b = (256.0 * clamp(sb, 0.0, 0.999)) as i32;

    println!("{} {} {}", r, g, b);
}

fn ray_color(r: Ray, hittable: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::zero();
    }
    match hittable.hit(r, 0.001, INFINITY) {
        Some(hit_record) => match hit_record.material.scatter(&r, &hit_record) {
            Some((attenuation, scattered)) => {
                attenuation * ray_color(scattered, hittable, depth - 1)
            }
            None => Color::zero(),
        },
        None => {
            let unit_direction = r.direction.normalized();
            let t = 0.5 * (unit_direction.y + 1.0);
            (1.0 - t) * Color::one() + t * Color::new(0.5, 0.7, 1.0)
        }
    }
}

fn main() {
    // Image
    // let aspect_ratio = 3.0 / 2.0;
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let mut world: Vec<SharedHittable> = Vec::new();
    let mut look_from = Point3::zero();
    let mut look_at = Point3::zero();
    let vup = Point3::new(0.0, 1.0, 0.0);
    let mut vfov = 40.0;
    let mut aperture = 0.0;

    let world_num = 4;
    match world_num {
        1 => {
            world = random_scene();
            look_from = Point3::new(13.0, 2.0, 3.0);
            look_at = Point3::zero();
            vfov = 20.0;
            aperture = 0.1;
        }
        2 => {
            world = two_spheres();
            look_from = Point3::new(13.0, 2.0, 3.0);
            look_at = Point3::zero();
            vfov = 20.0;
        }
        3 => {
            world = two_perlin_spheres();
            look_from = Point3::new(13.0, 2.0, 3.0);
            look_at = Point3::zero();
            vfov = 20.0;
        }
        4 => {
            world = earth();
            look_from = Point3::new(13.0, 2.0, 3.0);
            look_at = Point3::zero();
            vfov = 20.0;
        }
        _ => {}
    }

    // Camera
    let dist_to_focus = 10.0;
    let camera = Camera::new(
        look_from,
        look_at,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    let mut colors: Vec<(usize, Color)> = (0..image_height)
        .rev()
        .flat_map(|j| (0..image_width).zip(iter::repeat(j)))
        .enumerate()
        .par_bridge()
        .map(|(n, (i, j))| {
            let mut color = Color::zero();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rand()) / (image_width - 1) as f64;
                let v = (j as f64 + rand()) / (image_height - 1) as f64;

                let r = camera.get_ray(u, v);
                color = color + ray_color(r, &world, max_depth);
            }
            (n, color)
        })
        .collect();

    colors.sort_by_key(|(n, _)| *n);

    println!("P3\n{} {}\n255", image_width, image_height);
    for (_, color) in colors.iter() {
        write_color(&color, samples_per_pixel);
    }

    eprintln!("\nDone.");
}

fn random_scene() -> Vec<SharedHittable> {
    let mut world: Vec<SharedHittable> = Vec::new();

    let checker: SharedTexture = Arc::new(Checker::new(
        Arc::new(SolidColor::new(Color::new(0.2, 0.3, 0.1))),
        Arc::new(SolidColor::new(Color::new(0.9, 0.9, 0.9))),
    ));

    let ground_mat: SharedMaterial = Arc::new(Lambertian::new(checker));
    let ground = Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_mat,
    ));
    world.push(ground);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand();
            let center = Point3::new(a as f64 + 0.9 * rand(), 0.2, b as f64 + 0.9 * rand());

            if (center - Point3::new(4.0, 0.2, 0.0)).mag() > 0.9 {
                let sphere_mat: SharedMaterial = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let mat = Arc::new(Lambertian::new(Arc::new(SolidColor::new(albedo))));
                    let center2 = center + Vec3::new(0.0, rand_range(0.0, 0.5), 0.0);
                    world.push(Box::new(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        mat.clone(),
                    )));
                    mat
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = rand_range(0.0, 0.5);
                    Arc::new(Metal::new(albedo, fuzz))
                } else {
                    // glass
                    Arc::new(Dielectric::new(1.5))
                };

                let sphere = Box::new(Sphere::new(center, 0.2, sphere_mat));
                world.push(sphere);
            }
        }
    }

    world
}

fn two_spheres() -> Vec<SharedHittable> {
    let checker: SharedTexture = Arc::new(Checker::new(
        Arc::new(SolidColor::new(Color::new(0.2, 0.3, 0.1))),
        Arc::new(SolidColor::new(Color::new(0.9, 0.9, 0.9))),
    ));

    let world: Vec<SharedHittable> = vec![
        Box::new(Sphere::new(
            Point3::new(0.0, -10.0, 0.0),
            10.0,
            Arc::new(Lambertian::new(checker.clone())),
        )),
        Box::new(Sphere::new(
            Point3::new(0.0, 10.0, 0.0),
            10.0,
            Arc::new(Lambertian::new(checker)),
        )),
    ];

    world
}

fn two_perlin_spheres() -> Vec<SharedHittable> {
    let mut hittables: Vec<SharedHittable> = Vec::new();

    let noise = Arc::new(Noise::new(4.0));
    hittables.push(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(noise.clone())),
    )));
    hittables.push(Box::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new(noise)),
    )));

    hittables
}

fn earth() -> Vec<SharedHittable> {
    let earth_texture = Arc::new(Image::new("./earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian::new(earth_texture));
    let globe = Box::new(Sphere::new(Point3::zero(), 2.0, earth_surface));

    let world: Vec<SharedHittable> = vec![globe];
    world
}
