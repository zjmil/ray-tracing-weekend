use std::sync::Arc;

use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::moving_sphere::MovingSphere;
use crate::ray::Ray;
use crate::vec3::*;

pub struct Sphere {
    inner: MovingSphere,
}

impl Sphere {
    pub fn new(center: Point3, radius: Float, material: Arc<dyn Material>) -> Sphere {
        Sphere {
            inner: MovingSphere::new(center, center, 0.0, 1.0, radius, material.clone()),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        self.inner.hit(r, t_min, t_max)
    }

    fn bounding_box(&self, t0: Time, t1: Time) -> Option<AABB> {
        self.inner.bounding_box(t0, t1)
    }
}
