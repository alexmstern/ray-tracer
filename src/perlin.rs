use rand::{rngs::ThreadRng, Rng};
use crate::vector3::Vector3;

const POINT_COUNT: usize = 256;

#[derive(Clone)]
pub struct Perlin {
    randvec: Vec<Vector3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut randvec = vec![Vector3::new(0.0, 0.0, 0.0); POINT_COUNT];
        for i in 0..POINT_COUNT {
            randvec[i] = Vector3::random_unit_vector();
        }

        let perm_x = Perlin::generate_perm(&mut rng);
        let perm_y = Perlin::generate_perm(&mut rng);
        let perm_z = Perlin::generate_perm(&mut rng);

        Perlin { randvec, perm_x, perm_y, perm_z }
    }

    pub fn noise(&self, p: Vector3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as isize;
        let j = p.y().floor() as isize;
        let k = p.z().floor() as isize;

        let mut c = [[[Vector3::new(0.0,0.0,0.0); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.randvec[
                        self.perm_x[((i + di as isize) & 255) as usize] ^
                        self.perm_y[((j + dj as isize) & 255) as usize] ^
                        self.perm_z[((k + dk as isize) & 255) as usize]
                    ];
                }
            }
        }

        Perlin::trilinear_interp(c, u, v, w)
    }

    pub fn turb(&self, p: &Vector3, depth: usize) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p = temp_p * 2.0;
        }

        accum.abs()
    }

    fn generate_perm(rng: &mut ThreadRng) -> Vec<usize> {
        let mut p = (0..POINT_COUNT).collect::<Vec<_>>();
        Perlin::permute(&mut p, POINT_COUNT, rng);
        p
    }

    fn permute(p: &mut [usize], n: usize, rng: &mut ThreadRng) {
        for i in (1..n).rev() {
            let target = rng.gen_range(0..i+1);
            p.swap(i, target);
        }
    }

    fn trilinear_interp(c: [[[Vector3; 2]; 2]; 2] , u:f64, v:f64, w:f64) -> f64 {
        let uu = u*u*(3.0-2.0*u);
        let vv = v*v*(3.0-2.0*v);
        let ww = w*w*(3.0-2.0*w);
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vector3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum = accum + ((i as f64)*uu + (1.0 - i as f64)*(1.0-uu))
                                  * ((j as f64)*vv + (1.0 - j as f64)*(1.0-vv))
                                  * ((k as f64)*ww + (1.0 - k as f64)*(1.0-ww))
                                  * c[i][j][k].dot(weight_v);
                }
            }
        }
        accum
    }
}