use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable, SharedHittable};
use crate::material::SharedMaterial;
use crate::ray::Ray;
use crate::util::Point3;
use crate::util::Time;
use crate::vec3::Vec3;

pub struct Rect2D {
    v0: Vec3,
    v1: Vec3,
    k: f64,
    missing: MissingPos,
    material: SharedMaterial,
}

enum MissingPos {
    X,
    Y,
    Z,
}

impl MissingPos {
    fn idx(&self) -> usize {
        match self {
            X => 0,
            Y => 1,
            Z => 2,
        }
    }
}

impl Rect2D {
    fn new(
        v0: Vec3,
        v1: Vec3,
        k: f64,
        missing: MissingPos,
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
            MissingPos::Z,
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
            MissingPos::Y,
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
            MissingPos::X,
            material,
        )
    }
}

impl Hittable for Rect2D {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin[self.missing.idx()]) / r.direction[self.missing.idx()];
        if t < t_min || t > t_max {
            return None;
        }

        let xyz = r.origin + t * r.direction;

        for i in 0..3 {
            if i != self.missing.idx() {
                if xyz[i] < self.v0[i] || xyz[i] > self.v1[i] {
                    return None;
                }
            }
        }

        let scaled = (xyz - self.v0) / (self.v1 - self.v0);

        let (u, v, outward_normal) = match self.missing {
            MissingPos::X => (scaled.y, scaled.z, Vec3::new(1.0, 0.0, 0.0)),
            MissingPos::Y => (scaled.x, scaled.z, Vec3::new(0.0, 1.0, 0.0)),
            MissingPos::Z => (scaled.x, scaled.y, Vec3::new(0.0, 0.0, 1.0)),
        };

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
        let mut lower = self.v0;
        let mut upper = self.v1;

        lower[self.missing.idx()] = self.k - 0.0001;
        upper[self.missing.idx()] = self.k + 0.0001;
        Some(AABB::new(lower, upper))
    }
}
