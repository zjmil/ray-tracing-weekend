use image::io::Reader as ImageReader;
use image::{Pixel, RgbImage};

use crate::perlin::Perlin;
use crate::vec3::*;

pub trait Texture: Send + Sync {
    fn value(&self, u: Float, v: Float, p: &Point3) -> Color;
}

pub struct SolidColor {
    color: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> Self {
        SolidColor { color }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: Float, _v: Float, _p: &Point3) -> Color {
        self.color
    }
}

pub struct Checker<Odd, Even> {
    odd: Odd,
    even: Even,
}

impl<Odd, Even> Checker<Odd, Even> {
    pub fn new(odd: Odd, even: Even) -> Self {
        Checker { odd, even }
    }
}

impl<Odd: Texture, Even: Texture> Texture for Checker<Odd, Even> {
    fn value(&self, u: Float, v: Float, p: &Point3) -> Color {
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
    scale: Float,
}

impl Noise {
    pub fn new(scale: Float) -> Self {
        Noise {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for Noise {
    fn value(&self, _u: Float, _v: Float, p: &Point3) -> Color {
        Color::full(0.5) * (1.0 + (self.scale * p.z + 10.0 * self.noise.turbulence(p)).sin())
    }
}

pub struct Image {
    image: RgbImage,
}

impl Image {
    pub fn new(filename: &str) -> Self {
        let image = ImageReader::open(filename)
            .unwrap()
            .decode()
            .unwrap()
            .to_rgb8();
        Image { image }
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
