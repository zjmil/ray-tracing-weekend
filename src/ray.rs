use crate::util::{Point3, Time};
use crate::vec3::Vec3;

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
    pub time: Time,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3, time: Time) -> Ray {
        Ray {
            origin,
            direction,
            time,
        }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}
