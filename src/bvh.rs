
use crate::aabb::AABB;
use crate::hittable::{Hittable, SharedHittable, HitRecord};
use crate::util::*;
use rand::prelude::*;



pub struct BvhNode {
    left: SharedHittable,
    right: SharedHittable,
    bbox: AABB;
}


#[inline]
fn box_compare(a: &SharedHittable, b: &SharedHittable, axis: usize) -> Ordering {
    let a_hit = a.bounding_box(0.0, 0.0);
    let b_hit = b.bounding_box(0.0, 0.0);

    if a_hit.is_none() || b_hit.is_none() {
        eprintln!("No bounding box in BvhNode compare")
    }

    a_hit.unwrap_or(AABB::zero()).min[axis].cmp(b_hit.unwrap_or(AABB::zero()).min[axis])
}


impl BvhNode {
    pub fn new(left: SharedHittable, right: SharedHittable, bbox: AABB) -> BvhNode {
        BvhNode { left, right, bbox }
    }

    pub fn from_objects(t0: Time, t1: Time, objects: &[SharedHittable]) -> BvhNode {
        let axis = thread_rng().gen_range(0, 3);

        let comparator = |a, b| box_compare(a, b, axis);

        let (left, right) = if objects.len() == 1 {
            (objects[0], objects[0])
        } else {
            let mut v = Vec::clone_from_slize(objects);
            v.sort_by(comparator);
            let mid = objects.len() / 2;

            (objects[..mid], objects[mid..])
        }

        let left_hit = left.bounding_box(t0, t1);
        let right_hit = right.bounding_box(t0, t1);

        if left_hit.is_none() || right_hit.is_none() {
            eprintln!("No bounding box in BvhNode::from_objects")
        }

        let bbox = surrounding_box(left_hit.unwrap_or(AABB::zero()), right_hit.unwrap_or(AABB::zero()));

        BvhNode::new(left, right, bbox)
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.bbox.hit(r, t_min, t_max) {
            return None;
        }

        let left_hit = left.hit(r, t_min, t_max);
        let right_hit = right.hit(r, t_min, left_hit.map_or(t_max, |r| r.t));

        right_hit.or(left_hit)
    }
    fn bounding_box(&self, t0: Time, t1: Time) -> Option<AABB> {
        Some(self.box)
    }
}
