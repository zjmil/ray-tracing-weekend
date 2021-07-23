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

            t_min = t0.max(t_min);
            t_max = t1.min(t_max);
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
