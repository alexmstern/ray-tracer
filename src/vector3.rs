use std::ops::{Add, Div, Mul, Sub};

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