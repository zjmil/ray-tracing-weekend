mod aabb;
mod aarect;
mod camera;
mod cube;
mod hittable;
mod material;
mod moving_sphere;
mod perlin;
mod ray;
mod sphere;
mod texture;
mod util;
mod vec3;

use aarect::Rect2D;
use camera::Camera;
use cube::Cube;
use hittable::{Hittable, SharedHittable};
use material::*;
use moving_sphere::MovingSphere;
use ray::Ray;
use rayon::prelude::*;
use sphere::Sphere;
use std::env;
use std::iter;
use texture::*;
use util::*;
use vec3::*;

use std::f64::INFINITY;
use std::time::SystemTime;

fn write_color(color: &Color, samples_per_pixel: i32) {
    // Divide the color by the number of samples and gamma-correct for gamma=2.0
    let scale = 1.0 / (samples_per_pixel as f64);
    let sc = (scale * color).sqrt();
    let (fr, fg, fb) = (256.0 * sc.clamp(0.0, 0.999)).as_tuple();
    let (r, g, b) = (fr as i32, fg as i32, fb as i32);

    println!("{} {} {}", r, g, b);
}

fn ray_color(r: &Ray, background: &Color, world: &dyn Hittable, depth: i32) -> Color {
    // base case for ray bounce limit
    if depth <= 0 {
        return Color::zero();
    }

    if let Some(rec) = world.hit(r, 0.001, INFINITY) {
        let emitted = rec.material.emitted(rec.u, rec.v, &rec.p);

        if let Some((attenuation, scattered)) = rec.material.scatter(&r, &rec) {
            emitted + attenuation * ray_color(&scattered, background, world, depth - 1)
        } else {
            emitted
        }
    } else {
        *background
    }
}

struct ProgramArgs {
    scene: i32,
}

fn parse_arguments() -> ProgramArgs {
    let args: Vec<String> = env::args().collect();
    let mut it = args.iter();

    let mut args = ProgramArgs { scene: 1 };

    while let Some(val) = it.next() {
        if val == "-s" || val == "--scene" {
            args.scene = it.next().and_then(|s| s.parse().ok()).unwrap_or(args.scene);
        }
    }

    args
}

fn main() {
    let args = parse_arguments();

    // Image
    // let aspect_ratio = 3.0 / 2.0;
    let mut aspect_ratio = 16.0 / 9.0;
    let mut image_width = 400;
    let mut samples_per_pixel = 100;
    let max_depth = 50;

    let world: Vec<SharedHittable>;
    let look_from;
    let look_at;
    let vup = Point3::new(0.0, 1.0, 0.0);
    let vfov;
    let mut aperture = 0.0;
    let background;

    match args.scene {
        1 => {
            world = random_scene();
            background = Color::new(0.7, 0.8, 1.0);
            look_from = Point3::new(13.0, 2.0, 3.0);
            look_at = Point3::zero();
            vfov = 20.0;
            aperture = 0.1;
        }
        2 => {
            world = two_spheres();
            background = Color::new(0.7, 0.8, 1.0);
            look_from = Point3::new(13.0, 2.0, 3.0);
            look_at = Point3::zero();
            vfov = 20.0;
        }
        3 => {
            world = two_perlin_spheres();
            background = Color::new(0.7, 0.8, 1.0);
            look_from = Point3::new(13.0, 2.0, 3.0);
            look_at = Point3::zero();
            vfov = 20.0;
        }
        4 => {
            world = earth();
            background = Color::new(0.7, 0.8, 1.0);
            look_from = Point3::new(13.0, 2.0, 3.0);
            look_at = Point3::zero();
            vfov = 20.0;
        }
        5 => {
            world = simple_light();
            samples_per_pixel = 400;
            background = Color::zero();
            look_from = Point3::new(26.0, 3.0, 6.0);
            look_at = Point3::new(0.0, 2.0, 0.0);
            vfov = 20.0;
        }
        6 | _ => {
            world = cornell_box();
            aspect_ratio = 1.0;
            image_width = 600;
            samples_per_pixel = 200;
            background = Color::zero();
            look_from = Point3::new(278.0, 278.0, -800.0);
            look_at = Point3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
    }

    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // Camera
    let dist_to_focus = 10.0;
    let camera = Camera::new(
        &look_from,
        &look_at,
        &vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    let start_time = SystemTime::now();

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
                color += ray_color(&r, &background, &world, max_depth);
            }
            (n, color)
        })
        .collect();

    colors.sort_by_key(|(n, _)| *n);

    println!("P3\n{} {}\n255", image_width, image_height);
    for (_, color) in colors.iter() {
        write_color(&color, samples_per_pixel);
    }

    let total_time = start_time.elapsed().unwrap();

    eprintln!("\nDone. Seconds = {}", total_time.as_secs_f32());
}

fn random_scene() -> Vec<SharedHittable> {
    let mut world = Vec::new();

    let checker = Checker::new(
        SolidColor::new(Color::new(0.2, 0.3, 0.1)),
        SolidColor::new(Color::new(0.9, 0.9, 0.9)),
    );

    let ground_mat = Lambertian::new(checker);
    let ground = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_mat);
    world.push(ground);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand();
            let center = Point3::new(a as f64 + 0.9 * rand(), 0.2, b as f64 + 0.9 * rand());

            if (center - Point3::new(4.0, 0.2, 0.0)).mag() > 0.9 {
                let sphere_mat: SharedMaterial = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let mat = Lambertian::new(SolidColor::new(albedo));
                    let center2 = center + Vec3::new(0.0, rand_range(0.0, 0.5), 0.0);
                    world.push(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        mat.clone(),
                    ));
                    mat
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = rand_range(0.0, 0.5);
                    Metal::new(albedo, fuzz)
                } else {
                    // glass
                    Dielectric::new(1.5)
                };

                let sphere = Sphere::new(center, 0.2, sphere_mat);
                world.push(sphere);
            }
        }
    }

    world
}

fn two_spheres() -> Vec<SharedHittable> {
    let checker = Checker::new(
        SolidColor::new(Color::new(0.2, 0.3, 0.1)),
        SolidColor::new(Color::new(0.9, 0.9, 0.9)),
    );

    vec![
        Sphere::new(
            Point3::new(0.0, -10.0, 0.0),
            10.0,
            Lambertian::new(checker.clone()),
        ),
        Sphere::new(Point3::new(0.0, 10.0, 0.0), 10.0, Lambertian::new(checker)),
    ]
}

fn two_perlin_spheres() -> Vec<SharedHittable> {
    let noise = Noise::new(4.0);

    vec![
        Sphere::new(
            Point3::new(0.0, -1000.0, 0.0),
            1000.0,
            Lambertian::new(noise.clone()),
        ),
        Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, Lambertian::new(noise)),
    ]
}

fn earth() -> Vec<SharedHittable> {
    let earth_texture = Image::new("./earthmap.jpg");
    let earth_surface = Lambertian::new(earth_texture);

    vec![Sphere::new(Point3::zero(), 2.0, earth_surface)]
}

fn simple_light() -> Vec<SharedHittable> {
    let noise = Noise::new(4.0);
    let difflight = DiffuseLight::new(SolidColor::new(Color::new(4.0, 4.0, 4.0)));

    vec![
        Sphere::new(
            Point3::new(0.0, -1000.0, 0.0),
            1000.0,
            Lambertian::new(noise.clone()),
        ),
        Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, Lambertian::new(noise)),
        Rect2D::new_xy(3.0, 5.0, 1.0, 3.0, -2.0, difflight),
    ]
}

fn cornell_box() -> Vec<SharedHittable> {
    let red = Lambertian::new(SolidColor::new(Color::new(0.65, 0.05, 0.05)));
    let white = Lambertian::new(SolidColor::new(Color::new(0.73, 0.73, 0.73)));
    let green = Lambertian::new(SolidColor::new(Color::new(0.12, 0.45, 0.15)));
    let light = DiffuseLight::new(SolidColor::new(Color::new(15.0, 15.0, 15.0)));

    vec![
        Rect2D::new_yz(0.0, 555.0, 0.0, 555.0, 555.0, green),
        Rect2D::new_yz(0.0, 555.0, 0.0, 555.0, 0.0, red),
        Rect2D::new_xz(213.0, 343.0, 227.0, 332.0, 554.0, light),
        Rect2D::new_xz(0.0, 555.0, 0.0, 555.0, 0.0, white.clone()),
        Rect2D::new_xz(0.0, 555.0, 0.0, 555.0, 555.0, white.clone()),
        Rect2D::new_xy(0.0, 555.0, 0.0, 555.0, 555.0, white.clone()),
        Cube::new(
            Point3::new(130.0, 0.0, 65.0),
            Point3::new(295.0, 165.0, 230.0),
            white.clone(),
        ),
        Cube::new(
            Point3::new(265.0, 0.0, 295.0),
            Point3::new(430.0, 330.0, 460.0),
            white.clone(),
        ),
    ]
}
