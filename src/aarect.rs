use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable, SharedHittable};
use crate::material::SharedMaterial;
use crate::ray::Ray;
use crate::util::Point3;
use crate::util::Time;
use crate::vec3::Vec3;

pub struct XYRect {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
    material: SharedMaterial,
}

impl XYRect {
    pub fn new(
        x0: f64,
        x1: f64,
        y0: f64,
        y1: f64,
        k: f64,
        material: SharedMaterial,
    ) -> SharedHittable {
        Box::new(XYRect {
            x0,
            x1,
            y0,
            y1,
            k,
            material,
        })
    }
}

impl Hittable for XYRect {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin.z) / r.direction.z;
        if t < t_min || t > t_max {
            return None;
        }

        let x = r.origin.x + t * r.direction.x;
        let y = r.origin.y + t * r.direction.y;

        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        let p = r.at(t);

        Some(HitRecord::new(
            p,
            t,
            u,
            v,
            r,
            outward_normal,
            self.material.clone(),
        ))
    }

    fn bounding_box(&self, t0: Time, t1: Time) -> Option<AABB> {
        Some(AABB::new(
            Point3::new(self.x0, self.y0, self.k - 0.0001),
            Point3::new(self.x1, self.y1, self.k + 0.0001),
        ))
    }
}
