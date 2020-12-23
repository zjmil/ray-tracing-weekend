use crate::ray::Ray;
use crate::util::*;
use crate::vec3::Vec3;
use std::mem::swap;

pub struct AABB {
    pub min: Point3,
    pub max: Point3,
}

impl AABB {
    pub fn new(min: Point3, max: Point3) -> AABB {
        AABB { min, max }
    }

    pub fn zero() -> AABB {
        AABB::new(Point3::zero(), Point3::zero())
    }

    pub fn hit(&self, r: &Ray, t_min_init: f64, t_max_init: f64) -> bool {
        let mut t_min = t_min_init;
        let mut t_max = t_max_init;

        for a in 0..3 {
            let inv_d = 1.0 / r.direction[a];
            let mut t0 = (self.min[a] - r.origin[a]) * inv_d;
            let mut t1 = (self.max[a] - r.origin[a]) * inv_d;

            if inv_d < 0.0 {
                swap(&mut t0, &mut t1);
            }

            t_min = max(t0, t_min);
            t_max = min(t1, t_max);
            if t_max <= t_min {
                return false;
            }
        }

        true
    }
}

#[inline]
fn vec3_min(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3::new(min(u.x, v.x), min(u.y, v.y), min(u.z, v.z))
}

#[inline]
fn vec3_max(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3::new(max(u.x, v.x), max(u.y, v.y), max(u.z, v.z))
}

pub fn surrounding_box(a: &AABB, b: &AABB) -> AABB {
    let small = vec3_min(&a.min, &b.min);
    let large = vec3_max(&a.max, &b.max);
    AABB::new(small, large)
}
