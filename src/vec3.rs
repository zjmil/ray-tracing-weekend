use std::ops;

use crate::util::{rand, rand_range};

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f64) -> Vec3 {
        Vec3::new(t * self.x, t * self.y, t * self.z)
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        v * self
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f64) -> Vec3 {
        Vec3::new(self.x / t, self.y / t, self.z / t)
    }
}

pub fn dot(v: &Vec3, u: &Vec3) -> f64 {
    v.x * u.x + v.y * u.y + v.z * u.z
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn zero() -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }

    pub fn one() -> Vec3 {
        Vec3::new(1.0, 1.0, 1.0)
    }

    pub fn random() -> Vec3 {
        Vec3::new(rand(), rand(), rand())
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3::new(
            rand_range(min, max),
            rand_range(min, max),
            rand_range(min, max),
        )
    }

    pub fn mag(&self) -> f64 {
        self.mag_squared().sqrt()
    }

    pub fn mag_squared(&self) -> f64 {
        dot(self, self)
    }

    pub fn normalized(&self) -> Vec3 {
        *self / self.mag()
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn abs(&self) -> Vec3 {
        Vec3::new(self.x.abs(), self.y.abs(), self.z.abs())
    }

    pub fn near_zero(&self) -> bool {
        let eps = 1.0e-8;
        self.x.abs() < eps && self.y.abs() < eps && self.z.abs() < eps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fapprox_eq(a: f64, b: f64) -> bool {
        let eps = 0.0000001;
        let diff = (a - b).abs();
        diff < eps
    }

    fn vapprox_eq(v: Vec3, w: Vec3) -> bool {
        fapprox_eq(v.x(), w.x()) && fapprox_eq(v.y(), w.y()) && fapprox_eq(v.z(), w.z())
    }

    #[test]
    fn test_add() {
        assert!(vapprox_eq(
            Vec3::new(0.0, 1.0, 2.0) + Vec3::new(3.0, 2.0, 1.0),
            Vec3::new(3.0, 3.0, 3.0)
        ))
    }

    #[test]
    fn test_min() {
        assert!(vapprox_eq(
            Vec3::new(1.0, 2.0, 3.0) - Vec3::new(0.0, 1.0, 2.0),
            Vec3::new(1.0, 1.0, 1.0)
        ))
    }

    #[test]
    fn test_mul() {
        assert!(vapprox_eq(
            -3.0 * Vec3::new(1.0, 2.0, -3.0),
            Vec3::new(-3.0, -6.0, 9.0)
        ))
    }

    #[test]
    fn test_div() {}

    #[test]
    fn test_dot() {
        assert!(fapprox_eq(
            dot(&Vec3::new(1.0, 3.0, -5.0), &Vec3::new(4.0, -2.0, -1.0)),
            3.0
        ))
    }

    #[test]
    fn test_mag() {}

    #[test]
    fn test_normalized() {
        let v = Vec3::new(3.0, 4.0, 5.0);
        let mag = v.mag();
        assert!(fapprox_eq(mag * mag, 50.0))
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let v = Vec3::random_range(-1.0, 1.0);
        if v.mag_squared() < 1.0 {
            return v;
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().normalized()
}

pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if dot(&in_unit_sphere, &normal) > 0.0 {
        // same hemispere as normal
        in_unit_sphere
    } else {
        -1.0 * in_unit_sphere
    }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * dot(&v, &n) * n
}