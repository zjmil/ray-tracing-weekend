use rand::prelude::*;

use crate::vec3::Vec3;

pub type Point3 = Vec3;
pub type Color = Vec3;
pub type Time = f64;

#[inline]
pub fn rand() -> f64 {
    thread_rng().gen()
}

#[inline]
pub fn rand_range(min: f64, max: f64) -> f64 {
    thread_rng().gen_range(min, max)
}

#[inline]
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

#[inline]
pub fn min(a: f64, b: f64) -> f64 {
    if a < b {
        a
    } else {
        b
    }
}

#[inline]
pub fn max(a: f64, b: f64) -> f64 {
    if a > b {
        a
    } else {
        b
    }
}

#[inline]
pub fn square(x: f64) -> f64 {
    x.powi(2)
}
