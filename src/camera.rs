use rand::prelude::*;

use crate::ray::Ray;
use crate::vec3::*;

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    origin: Point3,
    lower_left: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    w: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: Float,
    time0: Time,
    time1: Time,
}

impl Camera {
    pub fn new(
        look_from: &Point3,
        look_at: &Point3,
        vup: &Vec3,
        vfov: Float,
        aspect_ratio: Float,
        aperture: Float,
        focus_dist: Float,
        time0: Time,
        time1: Time,
    ) -> Camera {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).normalized();
        let u = vup.cross(&w).normalized();
        let v = w.cross(&u);

        let origin = *look_from;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;
        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            lower_left,
            horizontal,
            vertical,
            w,
            u,
            v,
            lens_radius,
            time0,
            time1,
        }
    }

    pub fn get_ray(&self, s: Float, t: Float) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        let origin = self.origin + offset;
        let direction = self.lower_left + s * self.horizontal + t * self.vertical - origin;
        let time = thread_rng().gen_range(self.time0..self.time1);
        Ray::new(origin, direction, time)
    }
}
