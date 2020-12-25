use crate::aabb::AABB;
use crate::aarect::Rect2D;
use crate::hittable::{HitRecord, Hittable, SharedHittable};
use crate::material::SharedMaterial;
use crate::ray::Ray;
use crate::util::{Point3, Time};

pub struct Cube {
    min: Point3,
    max: Point3,
    sides: Vec<SharedHittable>,
}

impl Cube {
    pub fn new(min: Point3, max: Point3, material: SharedMaterial) -> SharedHittable {
        let sides = vec![
            Rect2D::new_xy(min.x, max.x, min.y, max.y, max.z, material.clone()),
            Rect2D::new_xy(min.x, max.x, min.y, max.y, min.z, material.clone()),
            Rect2D::new_xz(min.x, max.x, min.z, max.z, max.y, material.clone()),
            Rect2D::new_xz(min.x, max.x, min.z, max.z, min.y, material.clone()),
            Rect2D::new_yz(min.y, max.y, min.z, max.z, max.x, material.clone()),
            Rect2D::new_yz(min.y, max.y, min.z, max.z, min.x, material.clone()),
        ];

        Box::new(Cube { min, max, sides })
    }
}

impl Hittable for Cube {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }

    fn bounding_box(&self, _t0: Time, _t1: Time) -> Option<AABB> {
        Some(AABB::new(self.min, self.max))
    }
}
