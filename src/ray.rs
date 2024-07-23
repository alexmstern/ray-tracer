use crate::vector3::Vector3;
use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Ray<T> {
    pub orig: Vector3<T>,
    pub dir: Vector3<T>,
}

impl<T> Ray<T>
where
    T: Copy + ops::Add<Output = T> + ops::Mul<Output = T> + ops::Div<Output = T>,
{
    pub fn new(origin: Vector3<T>, direction: Vector3<T>) -> Self {
        Ray { orig: origin, dir: direction }
    }

    pub fn at(&self, t: T) -> Vector3<T> {
        self.orig + (self.dir * t)
    }
}
