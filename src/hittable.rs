use crate::aabb::{surrounding_box, AABB};
use crate::material::SharedMaterial;
use crate::ray::Ray;
use crate::util::Point3;
use crate::util::Time;
use crate::vec3::{dot, Vec3};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub material: SharedMaterial,
}

impl HitRecord {
    pub fn new(
        p: Point3,
        t: f64,
        u: f64,
        v: f64,
        ray: Ray,
        outward_normal: Vec3,
        material: SharedMaterial,
    ) -> HitRecord {
        let front_face = dot(ray.direction, outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
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

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, t0: Time, t1: Time) -> Option<AABB>;
}

pub type SharedHittable = Box<dyn Hittable + Send + Sync>;

impl Hittable for Vec<SharedHittable> {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut rec = None;

        for obj in self.iter() {
            if let Some(obj_rec) = obj.hit(r, t_min, closest_so_far) {
                closest_so_far = obj_rec.t;
                rec = Some(obj_rec.clone());
            }
        }
        rec
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
