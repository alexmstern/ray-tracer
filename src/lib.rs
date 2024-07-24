pub mod vector3;
pub mod ray;
pub mod hittable;
pub mod sphere;
pub mod material;
pub mod camera;

use rand::Rng;

pub fn random_double(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}