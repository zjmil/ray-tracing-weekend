use rand::prelude::*;

pub fn rand() -> f64 {
    thread_rng().gen()
}

pub fn rand_range(min: f64, max: f64) -> f64 {
    thread_rng().gen_range(min, max)
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

pub fn min(a: f64, b: f64) -> f64 {
    if a < b {
        a
    } else {
        b
    }
}

pub fn square(x: f64) -> f64 {
    x.powi(2)
}
