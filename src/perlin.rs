use crate::util::Point3;
use crate::vec3::*;
use rand::prelude::*;

pub struct Perlin {
    vecs: Vec<Vec3>,
    permx: Vec<usize>,
    permy: Vec<usize>,
    permz: Vec<usize>,
}

impl Perlin {
    pub fn new() -> Perlin {
        let mut rng = thread_rng();
        let size = 256;
        let vecs = (0..size).map(|_| Vec3::random_range(-1.0, 1.0)).collect();

        let mut permx: Vec<_> = (0..size).collect();
        permx.shuffle(&mut rng);
        let mut permy: Vec<_> = (0..size).collect();
        permy.shuffle(&mut rng);
        let mut permz: Vec<_> = (0..size).collect();
        permz.shuffle(&mut rng);

        Perlin {
            vecs,
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

        let mut c: [[[Vec3; 2]; 2]; 2] = [[[Vec3::zero(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let xi = ((i + di as i32) & 255) as usize;
                    let yi = ((j + dj as i32) & 255) as usize;
                    let zi = ((k + dk as i32) & 255) as usize;
                    let fi = self.permx[xi] ^ self.permy[yi] ^ self.permz[zi];
                    c[di][dj][dk] = self.vecs[fi];
                }
            }
        }

        let p = Vec3::new(u, v, w);
        Self::perlin_interp(&c, p)
    }

    /*
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
    */

    fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], p: Vec3) -> f64 {
        let pp = p * p * (Vec3::new(3.0, 3.0, 3.0) - 2.0 * p);
        let mut acc = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let ijk = Vec3::new(i as f64, j as f64, k as f64);

                    let weight_v = p - ijk;
                    acc += (ijk * pp + (Vec3::one() - ijk) * (Vec3::one() - pp)).product()
                        * dot(&c[i][j][k], &weight_v);
                }
            }
        }

        acc
    }

    pub fn turbulance(&self, p: Point3) -> f64 {
        let depth = 7;
        let mut temp_p = p;
        let mut acc = 0.0;
        let mut weight = 1.0;

        for _ in 0..depth {
            acc += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p = 2.0 * temp_p;
        }

        acc.abs()
    }
}
