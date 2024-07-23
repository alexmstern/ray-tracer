use crate::vector3::Vector3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    orig: Vector3,
    dir: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Self { Ray { orig: origin, dir: direction } }
    pub fn orig(&self) -> Vector3 { self.orig }
    pub fn dir(&self) -> Vector3 { self.dir }
    pub fn at(&self, t: f64) -> Vector3 { self.orig + (t * self.dir) }
}
