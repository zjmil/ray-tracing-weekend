use rand::prelude::*;

use crate::vec3::Vec3;

pub type Point3 = Vec3;
pub type Color = Vec3;
pub type Time = f64;

pub fn rand() -> f64 {
    thread_rng().gen()
}

pub fn rand_range(min: f64, max: f64) -> f64 {
    thread_rng().gen_range(min..max)
}
