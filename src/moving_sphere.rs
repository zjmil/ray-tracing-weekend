use crate::hittable::{HitRecord, Hittable};
use crate::material::SharedMaterial;
use crate::ray::Ray;
use crate::util::*;
use crate::vec3::*;

pub struct MovingSphere {
    center0: Point3,
    center1: Point3,
    time0: Time,
    time1: Time,
    radius: f64,
    material: SharedMaterial,
}

impl MovingSphere {
    pub fn new(
        center0: Point3,
        center1: Point3,
        time0: Time,
        time1: Time,
        radius: f64,
        material: SharedMaterial,
    ) -> MovingSphere {
        MovingSphere {
            center0,
            center1,
            time0,
            time1,
            radius,
            material,
        }
    }

    pub fn center(&self, time: Time) -> Point3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: Ray, t_min: Time, t_max: Time) -> Option<HitRecord> {
        let oc = r.origin - self.center(r.time);
        let a = r.direction.mag_squared();
        let half_b = dot(&oc, &r.direction);
        let c = oc.mag_squared() - square(self.radius);

        let discriminant = square(half_b) - a * c;
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
        let outward_normal = (at - self.center(r.time)) / self.radius;
        Some(HitRecord::new(
            at,
            root,
            r,
            &outward_normal,
            self.material.clone(),
        ))
    }
}
