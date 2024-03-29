use crate::aabb::{surrounding_box, AABB};
use crate::hittable::{HitRecord, Hittable, SharedHittable};
use crate::material::SharedMaterial;
use crate::ray::Ray;
use crate::util::*;
use crate::vec3::*;
use std::f64::consts::PI;

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
    ) -> SharedHittable {
        Box::new(MovingSphere {
            center0,
            center1,
            time0,
            time1,
            radius,
            material,
        })
    }

    pub fn center(&self, time: Time) -> Point3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

// TODO: remove copy
fn get_sphere_uv(p: &Point3) -> (f64, f64) {
    // p: a given point on the sphere of radius one, centered at the origin.
    // u: returned value [0,1] of angle around the Y axis from X=-1.
    // v: returned value [0,1] of angle from Y=-1 to Y=+1.
    //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
    //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
    //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>
    let theta = (-p.y).acos();
    let phi = (-p.z).atan2(-p.x) + PI;

    let u = phi / (2.0 * PI);
    let v = theta / PI;
    (u, v)
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: Time, t_max: Time) -> Option<HitRecord> {
        let oc = r.origin - self.center(r.time);
        let a = r.direction.mag_squared();
        let half_b = dot(&oc, &r.direction);
        let c = oc.mag_squared() - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
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
        let (u, v) = get_sphere_uv(&outward_normal);
        Some(HitRecord::new(
            at,
            root,
            u,
            v,
            r,
            &outward_normal,
            self.material.clone(),
        ))
    }

    fn bounding_box(&self, t0: Time, t1: Time) -> Option<AABB> {
        let rvec = Vec3::full(self.radius);
        let ct0 = self.center(t0);
        let ct1 = self.center(t1);
        let box0 = AABB::new(ct0 - rvec, ct0 + rvec);
        let box1 = AABB::new(ct1 - rvec, ct1 + rvec);

        Some(surrounding_box(&box0, &box1))
    }
}
