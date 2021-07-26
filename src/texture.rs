use crate::perlin::Perlin;
use crate::util::*;
use crate::vec3::Float;
use image::io::Reader as ImageReader;
use image::{Pixel, RgbImage};
use std::sync::Arc;

pub trait Texture {
    fn value(&self, u: Float, v: Float, p: &Point3) -> Color;
}

pub type SharedTexture = Arc<dyn Texture + Send + Sync>;

pub struct SolidColor {
    color: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> SharedTexture {
        Arc::new(SolidColor { color })
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: Float, _v: Float, _p: &Point3) -> Color {
        self.color
    }
}

pub struct Checker {
    odd: SharedTexture,
    even: SharedTexture,
}

impl Checker {
    pub fn new(odd: SharedTexture, even: SharedTexture) -> SharedTexture {
        Arc::new(Checker { odd, even })
    }
}

impl Texture for Checker {
    fn value(&self, u: Float, v: Float, p: &Point3) -> Color {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        let texture = if sines < 0.0 { &self.odd } else { &self.even };
        texture.value(u, v, p)
    }
}

pub struct Noise {
    noise: Perlin,
    scale: Float,
}

impl Noise {
    pub fn new(scale: Float) -> SharedTexture {
        Arc::new(Noise {
            noise: Perlin::new(),
            scale,
        })
    }
}

impl Texture for Noise {
    fn value(&self, _u: Float, _v: Float, p: &Point3) -> Color {
        Color::one() * 0.5 * (1.0 + (self.scale * p.z + 10.0 * self.noise.turbulence(p)).sin())
    }
}

pub struct Image {
    image: RgbImage,
}

impl Image {
    pub fn new(filename: &str) -> SharedTexture {
        let image = ImageReader::open(filename)
            .unwrap()
            .decode()
            .unwrap()
            .to_rgb8();
        Arc::new(Image { image })
    }
}

impl Texture for Image {
    fn value(&self, u: Float, v: Float, _p: &Point3) -> Color {
        let un = u.clamp(0.0, 1.0);
        let vn = 1.0 - v.clamp(0.0, 1.0);

        // Clamp coordinates
        let i = ((un * self.image.width() as Float) as u32).max(self.image.width() - 1);
        let j = ((vn * self.image.height() as Float) as u32).max(self.image.height() - 1);

        let pixel = self.image.get_pixel(i, j).to_rgb();
        let float_channels: Vec<_> = pixel.channels().iter().map(|x| *x as Float).collect();

        Color::from_slice(&float_channels) / 255.0
    }
}
