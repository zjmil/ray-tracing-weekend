use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{dot, Vec3};

use std::rc::Rc;

type Point3 = Vec3;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }

    pub fn center(&self) -> Point3 {
        self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().mag_squared();
        let half_b = dot(oc, r.direction());
        let c = oc.mag_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        // Find the nearest root that lies in the acceptable range
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let at = r.at(root);
        let outward_normal = (at - self.center) / self.radius;
        Some(HitRecord::new_normal(
            at,
            root,
            r,
            outward_normal,
            self.material.clone(),
        ))
    }
}
