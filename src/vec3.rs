use std::ops;
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::util::*;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

macro_rules! forward_ref_binop {
    (impl $imp:ident, $method:ident for $t:ty, $u:ty) => {
        impl<'a> $imp<$u> for &'a $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: $u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, other)
            }
        }

        impl<'a> $imp<&'a $u> for $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: &'a $u) -> <$t as $imp<$u>>::Output {
                $imp::$method(self, *other)
            }
        }

        impl<'a, 'b> $imp<&'a $u> for &'b $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: &'a $u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, *other)
            }
        }
    };
}

macro_rules! forward_ref_unop {
    (impl $imp:ident, $method:ident for $t:ty) => {
        impl<'a> $imp for &'a $t {
            type Output = <$t as $imp>::Output;

            #[inline]
            fn $method(self) -> <$t as $imp>::Output {
                $imp::$method(*self)
            }
        }
    };
}

impl Add for Vec3 {
    type Output = Vec3;

    #[inline]
    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    #[inline]
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, t: f64) -> Self {
        Self::new(t * self.x, t * self.y, t * self.z)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    #[inline]
    fn mul(self, v: Vec3) -> Vec3 {
        v * self
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    #[inline]
    fn div(self, t: f64) -> Self {
        Self::new(self.x / t, self.y / t, self.z / t)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    #[inline]
    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

forward_ref_binop! { impl Add, add for Vec3, Vec3 }
forward_ref_binop! { impl Sub, sub for Vec3, Vec3 }
forward_ref_binop! { impl Mul, mul for Vec3, Vec3 }
forward_ref_binop! { impl Mul, mul for Vec3, f64 }
forward_ref_binop! { impl Mul, mul for f64, Vec3 }
forward_ref_binop! { impl Div, div for Vec3, Vec3 }
forward_ref_binop! { impl Div, div for Vec3, f64}
forward_ref_unop! { impl Neg, neg for Vec3 }

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of range!"),
        }
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match idx {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of range!"),
        }
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn zero() -> Vec3 {
        Self::full(0.0)
    }

    pub fn one() -> Vec3 {
        Self::full(1.0)
    }

    pub fn full(a: f64) -> Vec3 {
        Self::new(a, a, a)
    }

    pub fn random() -> Vec3 {
        Self::new(rand(), rand(), rand())
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Self::new(
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
        self / self.mag()
    }

    pub fn as_tuple(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }

    pub fn floor(&self) -> Vec3 {
        Self::new(self.x.floor(), self.y.floor(), self.z.floor())
    }

    pub fn abs(&self) -> Vec3 {
        Self::new(self.x.abs(), self.y.abs(), self.z.abs())
    }

    pub fn sum(&self) -> f64 {
        self.x + self.y + self.z
    }

    pub fn product(&self) -> f64 {
        self.x * self.y * self.z
    }

    pub fn near_zero(&self) -> bool {
        let eps = 1.0e-8;
        self.x.abs() < eps && self.y.abs() < eps && self.z.abs() < eps
    }

    pub fn sqrt(&self) -> Vec3 {
        Vec3::new(self.x.sqrt(), self.y.sqrt(), self.z.sqrt())
    }

    pub fn clamp(&self, min: f64, max: f64) -> Vec3 {
        Vec3::new(
            self.x.clamp(min, max),
            self.y.clamp(min, max),
            self.z.clamp(min, max),
        )
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

pub fn dot(v: &Vec3, u: &Vec3) -> f64 {
    v.x * u.x + v.y * u.y + v.z * u.z
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - 2.0 * dot(v, n) * n
}

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = dot(&-uv, n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -(1.0 - r_out_perp.mag_squared()).abs().sqrt() * n;
    r_out_perp + r_out_parallel
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3::new(
        u.y * v.z - u.z * v.y,
        u.z * v.x - u.x * v.z,
        u.x * v.y - u.y * v.x,
    )
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(rand_range(-1.0, 1.0), rand_range(-1.0, 1.0), 0.0);
        if p.mag_squared() < 1.0 {
            return p;
        }
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
        fapprox_eq(v.x, w.x) && fapprox_eq(v.y, w.y) && fapprox_eq(v.z, w.z)
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
