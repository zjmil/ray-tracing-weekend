use crate::util::Point3;
use rand::distributions::Standard;
use rand::prelude::*;

pub struct Perlin {
    floats: Vec<f64>,
    permx: Vec<usize>,
    permy: Vec<usize>,
    permz: Vec<usize>,
}

impl Perlin {
    pub fn new() -> Perlin {
        let mut rng = thread_rng();
        let size = 256;
        let floats = rng.sample_iter(&Standard).take(size).collect();

        let mut permx: Vec<_> = (0..size).collect();
        permx.shuffle(&mut rng);
        let mut permy: Vec<_> = (0..size).collect();
        permy.shuffle(&mut rng);
        let mut permz: Vec<_> = (0..size).collect();
        permz.shuffle(&mut rng);

        Perlin {
            floats,
            permx,
            permy,
            permz,
        }
    }

    pub fn noise(&self, p: Point3) -> f64 {
        let parts = p - p.floor();
        // Hermitian Smoothing
        let parts_smoothed = parts * parts * (Point3::new(3.0, 3.0, 3.0) - 2.0 * parts);
        let (u, v, w) = parts_smoothed.as_tuple();

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;

        let mut c: [[[f64; 2]; 2]; 2] = [[[0.0; 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let xi = ((i + di as i32) & 255) as usize;
                    let yi = ((j + dj as i32) & 255) as usize;
                    let zi = ((k + dk as i32) & 255) as usize;
                    let fi = self.permx[xi] ^ self.permy[yi] ^ self.permz[zi];
                    c[di][dj][dk] = self.floats[fi];
                }
            }
        }

        Self::trilinear_inerp(&c, u, v, w)
    }

    fn trilinear_inerp(c: &[[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut acc = 0.0;

        for ii in 0..2 {
            for ji in 0..2 {
                for ki in 0..2 {
                    let i = ii as f64;
                    let j = ji as f64;
                    let k = ki as f64;

                    acc += (i * u + (1.0 - i) * (1.0 - u))
                        * (j * v + (1.0 - j) * (1.0 - v))
                        * (k * w + (1.0 - k) * (1.0 - w))
                        * c[ii][ji][ki];
                }
            }
        }

        acc
    }
}
