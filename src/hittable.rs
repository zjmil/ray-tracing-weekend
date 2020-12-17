use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{dot, Vec3};
use std::sync::Arc;

type Point3 = Vec3;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    t: f64,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new(p: Point3, normal: Vec3, t: f64, material: Arc<dyn Material>) -> HitRecord {
        HitRecord {
            p,
            normal,
            t,
            front_face: false,
            material,
        }
    }

    pub fn new_normal(
        p: Point3,
        t: f64,
        ray: Ray,
        outward_normal: &Vec3,
        material: Arc<dyn Material>,
    ) -> HitRecord {
        let front_face = dot(&ray.direction, outward_normal) < 0.0;
        let normal = if front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
        HitRecord {
            p,
            normal,
            t,
            front_face,
            material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl Hittable for Vec<Box<dyn Hittable + Send + Sync>> {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut rec: Option<HitRecord> = None;

        for obj in self.iter() {
            if let Some(obj_rec) = obj.hit(r, t_min, closest_so_far) {
                rec = Some(obj_rec.clone());
                closest_so_far = obj_rec.t;
            }
        }

        rec
    }
}
