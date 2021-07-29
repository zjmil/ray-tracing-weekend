use crate::util::{Point3, Time};
use crate::vec3::{Float, Vec3};

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
    pub time: Time,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3, time: Time) -> Self {
        Ray {
            origin,
            direction,
            time,
        }
    }

    pub fn at(&self, t: Float) -> Point3 {
        self.origin + t * self.direction
    }
}
