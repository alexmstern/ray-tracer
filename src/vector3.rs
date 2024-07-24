use std::ops::{Add, Div, Mul, Sub};

use crate::random_double;

#[derive(Clone, Debug, Copy)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 { Vector3 { x, y, z } }
    pub fn x(&self) -> f64 { self.x }
    pub fn y(&self) -> f64 { self.y }
    pub fn z(&self) -> f64 { self.z }
    pub fn dot(&self, other: Vector3) -> f64 { self.x * other.x() + self.y * other.y() + self.z * other.z() }
    pub fn length_squared(&self) -> f64 { self.dot(*self) }
    pub fn length(&self) -> f64 { self.length_squared().sqrt() }
    pub fn unit_vector(&self) -> Vector3 { *self / self.length() }
    pub fn random(min: f64, max: f64) -> Vector3 { Vector3::new(random_double(min, max), random_double(min, max), random_double(min, max)) }
    pub fn random_in_unit_sphere() -> Vector3 {
        loop {
            let p = Vector3::random(-1.0, 1.0);
            if p.length_squared() < 1.0 { return p }
        }
    }
    pub fn random_unit_vector() -> Vector3 { Vector3::random_in_unit_sphere().unit_vector() }
    pub fn random_on_hemisphere(normal: Vector3) -> Vector3 {
        let p = Vector3::random_unit_vector();
        if p.dot(normal) > 0.0 { p } else { -1.0 * p }
    }
    pub fn near_zero(&self) -> bool { self.x.abs() < 1e-8 && self.y.abs() < 1e-8 && self.z.abs() < 1e-8 }
    pub fn reflect(&self, n: Vector3) -> Vector3 { *self - 2.0*self.dot(n)*n }
    pub fn refract(&self, n: Vector3, etai_over_etat: f64) -> Vector3 {
        let cos_theta = f64::min(-1.0 * self.dot(n), 1.0);
        let r_out_perp = etai_over_etat * (*self + cos_theta*n);
        let r_out_parallel = (1.0 - r_out_perp.length_squared()).abs().sqrt() * (-1.0 * n);
        r_out_perp + r_out_parallel
    }
}

impl Default for Vector3 {
    fn default() -> Self { Vector3::new(0.0, 0.0, 0.0) }
}

impl Add for Vector3 {
    type Output = Vector3;
    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x(),
            y: self.y + other.y(),
            z: self.z + other.z(),
        }
    }
}

impl Sub for Vector3 {
    type Output = Vector3;
    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x(),
            y: self.y - other.y(),
            z: self.z - other.z(),
        }
    }
}

impl Mul for Vector3 {
    type Output = Vector3;
    fn mul(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;
    fn mul(self, scalar: f64) -> Vector3 {
        Vector3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Mul<Vector3> for f64 {
    type Output = Vector3;
    fn mul(self, vector: Vector3) -> Vector3 {
        vector * self
    }
}

impl Div<f64> for Vector3 {
    type Output = Vector3;
    fn div(self, scalar: f64) -> Vector3 {
        Vector3 {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}