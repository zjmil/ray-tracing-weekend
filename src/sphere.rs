use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable, SharedHittable};
use crate::material::SharedMaterial;
use crate::ray::Ray;
use crate::util::{Point3, Time};
use crate::vec3::{Float, Vec3};
use std::f32::consts::{PI, TAU};

pub struct Sphere {
    center: Point3,
    radius: Float,
    material: SharedMaterial,
}

impl Sphere {
    pub fn new(center: Point3, radius: Float, material: SharedMaterial) -> SharedHittable {
        Box::new(Sphere {
            center,
            radius,
            material,
        })
    }
}

fn get_sphere_uv(p: &Point3) -> (Float, Float) {
    // p: a given point on the sphere of radius one, centered at the origin.
    // u: returned value [0,1] of angle around the Y axis from X=-1.
    // v: returned value [0,1] of angle from Y=-1 to Y=+1.
    //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
    //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
    //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>
    let theta = (-p.y).acos();
    let phi = (-p.z).atan2(-p.x) + PI;

    let u = phi / TAU;
    let v = theta / PI;
    (u, v)
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.mag_squared();
        let half_b = oc.dot(&r.direction);
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
        let outward_normal = (at - self.center) / self.radius;
        let (u, v) = get_sphere_uv(&outward_normal);
        Some(HitRecord::new(
            at,
            root,
            u,
            v,
            &r,
            &outward_normal,
            self.material.clone(),
        ))
    }

    fn bounding_box(&self, _t0: Time, _t1: Time) -> Option<AABB> {
        let rad_vec = Vec3::full(self.radius);
        Some(AABB::new(self.center - rad_vec, self.center + rad_vec))
    }
}
