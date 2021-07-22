use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable, SharedHittable};
use crate::material::SharedMaterial;
use crate::ray::Ray;
use crate::util::Time;
use crate::vec3::Vec3;

pub struct Rect2D {
    v0: Vec3,
    v1: Vec3,
    k: f64,
    missing: Missing,
    material: SharedMaterial,
}

enum Missing {
    X,
    Y,
    Z,
}

impl Missing {
    fn idx(&self) -> usize {
        match self {
            Self::X => 0,
            Self::Y => 1,
            Self::Z => 2,
        }
    }
}

impl Rect2D {
    fn new(
        v0: Vec3,
        v1: Vec3,
        k: f64,
        missing: Missing,
        material: SharedMaterial,
    ) -> SharedHittable {
        Box::new(Rect2D {
            v0,
            v1,
            k,
            missing,
            material,
        })
    }

    pub fn new_xy(
        x0: f64,
        x1: f64,
        y0: f64,
        y1: f64,
        k: f64,
        material: SharedMaterial,
    ) -> SharedHittable {
        Self::new(
            Vec3::new(x0, y0, 0.0),
            Vec3::new(x1, y1, 0.0),
            k,
            Missing::Z,
            material,
        )
    }

    pub fn new_xz(
        x0: f64,
        x1: f64,
        z0: f64,
        z1: f64,
        k: f64,
        material: SharedMaterial,
    ) -> SharedHittable {
        Self::new(
            Vec3::new(x0, 0.0, z0),
            Vec3::new(x1, 0.0, z1),
            k,
            Missing::Y,
            material,
        )
    }

    pub fn new_yz(
        y0: f64,
        y1: f64,
        z0: f64,
        z1: f64,
        k: f64,
        material: SharedMaterial,
    ) -> SharedHittable {
        Self::new(
            Vec3::new(0.0, y0, z0),
            Vec3::new(0.0, y1, z1),
            k,
            Missing::X,
            material,
        )
    }
}

impl Hittable for Rect2D {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // TODO: refactor
        let (t, u, v, outward_normal) = match self.missing {
            Missing::Z => {
                let t = (self.k - r.origin.z) / r.direction.z;
                if t < t_min || t > t_max {
                    return None;
                }

                let x = r.origin.x + t * r.direction.x;
                let y = r.origin.y + t * r.direction.y;
                if x < self.v0.x || x > self.v1.x || y < self.v0.y || y > self.v1.y {
                    return None;
                }

                let u = (x - self.v0.x) / (self.v1.x - self.v0.x);
                let v = (y - self.v0.y) / (self.v1.y - self.v0.y);
                let outward_normal = Vec3::new(0.0, 0.0, 1.0);
                (t, u, v, outward_normal)
            }
            Missing::Y => {
                let t = (self.k - r.origin.y) / r.direction.y;
                if t < t_min || t > t_max {
                    return None;
                }

                let x = r.origin.x + t * r.direction.x;
                let z = r.origin.z + t * r.direction.z;
                if x < self.v0.x || x > self.v1.x || z < self.v0.z || z > self.v1.z {
                    return None;
                }

                let u = (x - self.v0.x) / (self.v1.x - self.v0.x);
                let v = (z - self.v0.z) / (self.v1.z - self.v0.z);
                let outward_normal = Vec3::new(0.0, 1.0, 0.0);
                (t, u, v, outward_normal)
            }
            Missing::X => {
                let t = (self.k - r.origin.x) / r.direction.x;
                if t < t_min || t > t_max {
                    return None;
                }

                let y = r.origin.y + t * r.direction.y;
                let z = r.origin.z + t * r.direction.z;
                if y < self.v0.y || y > self.v1.y || z < self.v0.z || z > self.v1.z {
                    return None;
                }

                let u = (y - self.v0.y) / (self.v1.y - self.v0.y);
                let v = (z - self.v0.z) / (self.v1.z - self.v0.z);
                let outward_normal = Vec3::new(1.0, 0.0, 0.0);
                (t, u, v, outward_normal)
            }
        };

        let p = r.at(t);
        Some(HitRecord::new(
            p,
            t,
            u,
            v,
            r,
            &outward_normal,
            self.material.clone(),
        ))
    }

    fn bounding_box(&self, _t0: Time, _t1: Time) -> Option<AABB> {
        let mut lower = self.v0;
        let mut upper = self.v1;

        lower[self.missing.idx()] = self.k - 0.0001;
        upper[self.missing.idx()] = self.k + 0.0001;
        Some(AABB::new(lower, upper))
    }
}
