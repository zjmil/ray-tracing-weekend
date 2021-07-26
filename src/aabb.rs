use crate::ray::Ray;
use crate::util::*;
use crate::vec3::Float;
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

    pub fn hit(&self, r: &Ray, t_min_init: Float, t_max_init: Float) -> bool {
        let mut t_min = t_min_init;
        let mut t_max = t_max_init;

        let inv_d = Vec3::one() / r.direction;
        let mut t0 = (self.min - r.origin) * inv_d;
        let mut t1 = (self.max - r.origin) * inv_d;

        for a in 0..3 {
            if inv_d[a] < 0.0 {
                swap(&mut t0[a], &mut t1[a]);
            }

            t_min = t0[a].max(t_min);
            t_max = t1[a].min(t_max);
            if t_max <= t_min {
                return false;
            }
        }

        true
    }
}

fn vec3_min(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3::new(u.x.min(v.x), u.y.min(v.y), u.z.min(v.z))
}

fn vec3_max(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3::new(u.x.max(v.x), u.y.max(v.y), u.z.max(v.z))
}

pub fn surrounding_box(a: &AABB, b: &AABB) -> AABB {
    let small = vec3_min(&a.min, &b.min);
    let large = vec3_max(&a.max, &b.max);
    AABB::new(small, large)
}
