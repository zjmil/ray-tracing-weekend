use std::env;
use std::iter;
use std::sync::Arc;
use std::time::SystemTime;

use rand::prelude::*;
use rayon::prelude::*;

use rtw::aarect::Rect2D;
use rtw::camera::Camera;
use rtw::cube::Cube;
use rtw::hittable::Hittable;
use rtw::material::*;
use rtw::moving_sphere::MovingSphere;
use rtw::ray::Ray;
use rtw::sphere::Sphere;
use rtw::texture::*;
use rtw::vec3::*;

fn write_color(color: &Color, samples_per_pixel: i32) {
    // Divide the color by the number of samples and gamma-correct for gamma=2.0
    let scale = 1.0 / (samples_per_pixel as Float);
    let sc = (scale * color).sqrt();
    let (fr, fg, fb) = (256.0 * sc.clamp(0.0, 0.999)).as_tuple();
    let (r, g, b) = (fr as i32, fg as i32, fb as i32);

    println!("{} {} {}", r, g, b);
}

fn ray_color(ray: &Ray, background: &Color, world: &dyn Hittable, depth: i32) -> Color {
    // base case for ray bounce limit
    if depth <= 0 {
        return Color::zero();
    }

    if let Some(rec) = world.hit(ray, 0.001, FLOAT_INF) {
        let offset = if let Some((attenuation, scattered)) = rec.material.scatter(ray, &rec) {
            attenuation * ray_color(&scattered, background, world, depth - 1)
        } else {
            Color::zero()
        };

        let emitted = rec.material.emitted(rec.u, rec.v, &rec.p);
        emitted + offset
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
    let max_depth = 25;

    let world: Vec<Arc<dyn Hittable>>;
    let look_from;
    let look_at;
    let vup = Point3::new(0.0, 1.0, 0.0);
    let vfov;
    let aperture;
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
            aperture = 0.0;
        }
        3 => {
            world = two_perlin_spheres();
            background = Color::new(0.7, 0.8, 1.0);
            look_from = Point3::new(13.0, 2.0, 3.0);
            look_at = Point3::zero();
            vfov = 20.0;
            aperture = 0.0;
        }
        4 => {
            world = earth();
            background = Color::new(0.7, 0.8, 1.0);
            look_from = Point3::new(13.0, 2.0, 3.0);
            look_at = Point3::zero();
            vfov = 20.0;
            aperture = 0.0;
        }
        5 => {
            world = simple_light();
            samples_per_pixel = 400;
            background = Color::zero();
            look_from = Point3::new(26.0, 3.0, 6.0);
            look_at = Point3::new(0.0, 2.0, 0.0);
            vfov = 20.0;
            aperture = 0.0;
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
            aperture = 0.0;
        }
    }

    let image_height = (image_width as Float / aspect_ratio) as i32;

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
            let mut rng = thread_rng();
            let mut color = Color::zero();
            for _ in 0..samples_per_pixel {
                let u = (i as Float + rng.gen::<Float>()) / (image_width - 1) as Float;
                let v = (j as Float + rng.gen::<Float>()) / (image_height - 1) as Float;

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

fn random_scene() -> Vec<Arc<dyn Hittable>> {
    let mut world: Vec<Arc<dyn Hittable>> = Vec::new();

    let checker = Checker::new(
        SolidColor::new(Color::new(0.2, 0.3, 0.1)),
        SolidColor::new(Color::new(0.9, 0.9, 0.9)),
    );

    let ground_mat = Lambertian::new(checker);
    let ground = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, Arc::new(ground_mat));
    world.push(Arc::new(ground));

    let mut rng = thread_rng();

    let n = 5;

    for a in -n..n {
        for b in -n..n {
            let choose_mat: Float = rng.gen();
            let xoffset: Float = 0.9 * rng.gen::<Float>();
            let zoffset: Float = 0.9 * rng.gen::<Float>();
            let center = Point3::new(a as Float + xoffset, 0.2, b as Float + zoffset);

            if (center - Point3::new(4.0, 0.2, 0.0)).mag() > 0.9 {
                let sphere_mat: Arc<dyn Material> = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let mat = Arc::new(Lambertian::new(SolidColor::new(albedo)));
                    let center2 = center + Vec3::new(0.0, rng.gen_range(0.0..0.5), 0.0);
                    let sphere = MovingSphere::new(center, center2, 0.0, 1.0, 0.2, mat.clone());
                    world.push(Arc::new(sphere));
                    mat
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    Arc::new(Metal::new(albedo, fuzz))
                } else {
                    // glass
                    Arc::new(Dielectric::new(1.5))
                };

                let sphere = Sphere::new(center, 0.2, sphere_mat);
                world.push(Arc::new(sphere));
            }
        }
    }

    world
}

fn two_spheres() -> Vec<Arc<dyn Hittable>> {
    let checker = Checker::new(
        SolidColor::new(Color::new(0.2, 0.3, 0.1)),
        SolidColor::new(Color::new(0.9, 0.9, 0.9)),
    );

    let lambertian = Arc::new(Lambertian::new(checker));

    vec![
        Arc::new(Sphere::new(
            Point3::new(0.0, -10.0, 0.0),
            10.0,
            lambertian.clone(),
        )),
        Arc::new(Sphere::new(Point3::new(0.0, 10.0, 0.0), 10.0, lambertian)),
    ]
}

fn two_perlin_spheres() -> Vec<Arc<dyn Hittable>> {
    let noise = Noise::new(4.0);

    let lambertian = Arc::new(Lambertian::new(noise));
    vec![
        Arc::new(Sphere::new(
            Point3::new(0.0, -1000.0, 0.0),
            1000.0,
            lambertian.clone(),
        )),
        Arc::new(Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, lambertian)),
    ]
}

fn earth() -> Vec<Arc<dyn Hittable>> {
    let earth_texture = Image::new("./earthmap.jpg");
    let earth_surface = Lambertian::new(earth_texture);

    vec![Arc::new(Sphere::new(
        Point3::zero(),
        2.0,
        Arc::new(earth_surface),
    ))]
}

fn simple_light() -> Vec<Arc<dyn Hittable>> {
    let noise = Noise::new(4.0);
    let difflight = DiffuseLight::new(SolidColor::new(Color::new(4.0, 4.0, 4.0)));

    let lambertian = Arc::new(Lambertian::new(noise));
    vec![
        Arc::new(Sphere::new(
            Point3::new(0.0, -1000.0, 0.0),
            1000.0,
            lambertian.clone(),
        )),
        Arc::new(Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, lambertian)),
        Arc::new(Rect2D::new_xy(
            3.0,
            5.0,
            1.0,
            3.0,
            -2.0,
            Arc::new(difflight),
        )),
    ]
}

fn cornell_box() -> Vec<Arc<dyn Hittable>> {
    let red = Arc::new(Lambertian::new(SolidColor::new(Color::new(
        0.65, 0.05, 0.05,
    ))));
    let white = Arc::new(Lambertian::new(SolidColor::new(Color::new(
        0.73, 0.73, 0.73,
    ))));
    let green = Arc::new(Lambertian::new(SolidColor::new(Color::new(
        0.12, 0.45, 0.15,
    ))));
    let light = Arc::new(DiffuseLight::new(SolidColor::new(Color::new(
        15.0, 15.0, 15.0,
    ))));

    vec![
        Arc::new(Rect2D::new_yz(0.0, 555.0, 0.0, 555.0, 555.0, green)),
        Arc::new(Rect2D::new_yz(0.0, 555.0, 0.0, 555.0, 0.0, red)),
        Arc::new(Rect2D::new_xz(213.0, 343.0, 227.0, 332.0, 554.0, light)),
        Arc::new(Rect2D::new_xz(0.0, 555.0, 0.0, 555.0, 0.0, white.clone())),
        Arc::new(Rect2D::new_xz(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())),
        Arc::new(Rect2D::new_xy(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())),
        Arc::new(Cube::new(
            Point3::new(130.0, 0.0, 65.0),
            Point3::new(295.0, 165.0, 230.0),
            white.clone(),
        )),
        Arc::new(Cube::new(
            Point3::new(265.0, 0.0, 295.0),
            Point3::new(430.0, 330.0, 460.0),
            white,
        )),
    ]
}
