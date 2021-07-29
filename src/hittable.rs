use crate::aabb::{surrounding_box, AABB};
use crate::material::Material;
use crate::ray::Ray;
use crate::util::Point3;
use crate::util::Time;
use crate::vec3::Float;
use crate::vec3::Vec3;
use std::sync::Arc;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: Float,
    pub u: Float,
    pub v: Float,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new(
        p: Point3,
        t: Float,
        u: Float,
        v: Float,
        ray: &Ray,
        outward_normal: &Vec3,
        material: Arc<dyn Material>,
    ) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            *outward_normal
        } else {
            -outward_normal
        };
        HitRecord {
            p,
            normal,
            t,
            u,
            v,
            front_face,
            material,
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord>;
    fn bounding_box(&self, t0: Time, t1: Time) -> Option<AABB>;
}

impl Hittable for Vec<Arc<dyn Hittable>> {
    fn hit(&self, r: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        self.iter()
            .fold((t_max, None), |(closest_so_far, rec), obj| {
                obj.hit(r, t_min, closest_so_far)
                    .map_or((closest_so_far, rec), |obj_rec| (obj_rec.t, Some(obj_rec)))
            })
            .1
    }

    fn bounding_box(&self, t0: Time, t1: Time) -> Option<AABB> {
        let mut acc = None;

        for obj in self.iter() {
            match obj.bounding_box(t0, t1) {
                Some(bbox) => {
                    acc = match acc {
                        Some(curr) => Some(surrounding_box(&curr, &bbox)),
                        None => Some(bbox),
                    }
                }
                None => return None,
            }
        }
        acc
    }
}
