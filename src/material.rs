use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::texture::{SharedTexture, SolidColor};
use crate::util::*;
use crate::vec3::*;
use std::sync::Arc;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: SharedTexture,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        let texture: SharedTexture = Arc::new(SolidColor::new(albedo));
        Lambertian::newt(texture)
    }

    pub fn newt(albedo: SharedTexture) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction, r_in.time);
        let attenuation = self.albedo.value(rec.u, rec.v, rec.p);
        Some((attenuation, scattered))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal {
            albedo,
            fuzz: clamp(fuzz, fuzz, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = reflect(&r_in.direction.normalized(), &rec.normal);
        let scattered = Ray::new(
            rec.p,
            reflected + self.fuzz * random_in_unit_sphere(),
            r_in.time,
        );

        if dot(&scattered.direction, &rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    index_of_refraction: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Dielectric {
        Dielectric {
            index_of_refraction,
        }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let r0 = square((1.0 - ref_idx) / (1.0 + ref_idx));
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let unit_direction = r_in.direction.normalized();

        let cos_theta = min(dot(&-unit_direction, &rec.normal), 1.0);
        let sin_theta = (1.0 - square(cos_theta)).sqrt();

        let direction = if refraction_ratio * sin_theta > 1.0
            || Dielectric::reflectance(cos_theta, refraction_ratio) > rand()
        {
            reflect(&unit_direction, &rec.normal)
        } else {
            refract(&unit_direction, &rec.normal, refraction_ratio)
        };

        let scattered = Ray::new(rec.p, direction, r_in.time);
        Some((Color::one(), scattered))
    }
}

pub type SharedMaterial = Arc<dyn Material + Sync + Send>;
