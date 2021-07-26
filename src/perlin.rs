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

    pub fn noise(&self, p: &Point3) -> Float {
        let parts = p - p.floor();
        // Hermitian Smoothing
        let parts_smoothed = parts * parts * (Point3::full(3.0) - 2.0 * parts);

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

        Self::perlin_interpolation(&c, &parts_smoothed)
    }

    fn perlin_interpolation(c: &[[[Vec3; 2]; 2]; 2], p: &Vec3) -> Float {
        let pp = p * p * (Vec3::full(3.0) - 2.0 * p);
        let mut acc = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let ijk = Vec3::new(i as Float, j as Float, k as Float);

                    let weight_v = p - ijk;
                    acc += (ijk * pp + (Vec3::one() - ijk) * (Vec3::one() - pp)).product()
                        * c[i][j][k].dot(&weight_v);
                }
            }
        }

        acc
    }

    pub fn turbulence(&self, p: &Point3) -> Float {
        let depth = 7;
        let mut temp_p = *p;
        let mut acc = 0.0;
        let mut weight = 1.0;

        for _ in 0..depth {
            acc += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p = 2.0 * temp_p;
        }

        acc.abs()
    }
}
