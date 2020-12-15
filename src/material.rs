use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::*;

type Color = Vec3;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal() + random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal();
        }

        let scattered = Ray::new(rec.p(), scatter_direction);
        Some((self.albedo, scattered))
    }
}

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Metal {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = reflect(r_in.direction().normalized(), rec.normal());
        let scattered = Ray::new(rec.p(), reflected);

        if dot(&scattered.direction(), &rec.normal()) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
