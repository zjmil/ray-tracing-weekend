use std::sync::Arc;

use crate::aabb::AABB;
use crate::aarect::Rect2D;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::*;

pub struct Cube {
    min: Point3,
    max: Point3,
    sides: [Rect2D; 6],
}

impl Cube {
    pub fn new(min: Point3, max: Point3, material: Arc<dyn Material>) -> Cube {
        let sides = [
            Rect2D::new_xy(min.x, max.x, min.y, max.y, max.z, material.clone()),
            Rect2D::new_xy(min.x, max.x, min.y, max.y, min.z, material.clone()),
            Rect2D::new_xz(min.x, max.x, min.z, max.z, max.y, material.clone()),
            Rect2D::new_xz(min.x, max.x, min.z, max.z, min.y, material.clone()),
            Rect2D::new_yz(min.y, max.y, min.z, max.z, max.x, material.clone()),
            Rect2D::new_yz(min.y, max.y, min.z, max.z, min.x, material.clone()),
        ];

        Cube { min, max, sides }
    }
}

impl Hittable for Cube {
    fn hit(&self, r: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        // TODO: copied, figure out way to implement generic
        self.sides
            .iter()
            .fold((t_max, None), |(closest_so_far, rec), obj| {
                obj.hit(r, t_min, closest_so_far)
                    .map_or((closest_so_far, rec), |obj_rec| (obj_rec.t, Some(obj_rec)))
            })
            .1
    }

    fn bounding_box(&self, _t0: Time, _t1: Time) -> Option<AABB> {
        Some(AABB::new(self.min, self.max))
    }
}
