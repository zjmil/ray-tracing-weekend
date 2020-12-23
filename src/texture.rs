use crate::perlin::Perlin;
use crate::util::*;
use std::sync::Arc;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color;
}

pub type SharedTexture = Arc<dyn Texture + Send + Sync>;

pub struct SolidColor {
    color: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> SolidColor {
        SolidColor { color }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: Point3) -> Color {
        self.color
    }
}

pub struct Checker {
    odd: SharedTexture,
    even: SharedTexture,
}

impl Checker {
    pub fn new(odd: SharedTexture, even: SharedTexture) -> Checker {
        Checker { odd, even }
    }
}

impl Texture for Checker {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

pub struct Noise {
    noise: Perlin,
    scale: f64,
}

impl Noise {
    pub fn new(scale: f64) -> Noise {
        Noise {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for Noise {
    fn value(&self, _u: f64, _v: f64, p: Point3) -> Color {
        Color::one() * self.noise.noise(self.scale * p)
    }
}
