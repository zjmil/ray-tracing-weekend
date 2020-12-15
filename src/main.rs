mod camera;
mod hittable;
mod material;
mod ray;
mod sphere;
mod util;
mod vec3;

use camera::Camera;
use hittable::Hittable;
use material::*;
use ray::Ray;
use sphere::Sphere;
use std::rc::Rc;
use util::*;
use vec3::*;

use std::f64::INFINITY;

type Color = Vec3;
type Point3 = Vec3;

fn write_color(color: &Color, samples_per_pixel: i32) {
    // Divide the color by the number of samples and gamma-coorect for gamma=2.0
    let scale = 1.0 / (samples_per_pixel as f64);
    let sr = (scale * color.x()).sqrt();
    let sg = (scale * color.y()).sqrt();
    let sb = (scale * color.z()).sqrt();

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
        Some(hit_record) => match hit_record.material().scatter(r, &hit_record) {
            Some((attenuation, scattered)) => {
                attenuation * ray_color(scattered, hittable, depth - 1)
            }
            None => Color::zero(),
        },
        None => {
            let unit_direction = r.direction().normalized();
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Color::one() + t * Color::new(0.5, 0.7, 1.0)
        }
    }
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let mat_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let mat_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let mat_left = Rc::new(Dielectric::new(1.5));
    let mat_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.8), 1.0));

    // World
    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(
            Point3::new(0.0, -100.5, -1.0),
            100.0,
            mat_ground,
        )),
        Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, mat_center)),
        Box::new(Sphere::new(
            Point3::new(-1.0, 0.0, -1.0),
            0.5,
            mat_left.clone(),
        )),
        Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), -0.4, mat_left)),
        Box::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, mat_right)),
    ];

    // Camera
    let camera = Camera::new();

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let mut color = Color::zero();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rand()) / (image_width - 1) as f64;
                let v = (j as f64 + rand()) / (image_height - 1) as f64;

                let r = camera.get_ray(u, v);
                color = color + ray_color(r, &world, max_depth);
            }

            write_color(&color, samples_per_pixel);
        }
    }

    eprintln!("\nDone.");
}
