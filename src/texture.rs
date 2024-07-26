use std::sync::Arc;
use crate::vector3::Vector3;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Vector3) -> Vector3;
}

#[derive(Clone, Copy)]
pub struct SolidColor {
    color_value: Vector3
}

impl SolidColor {
    pub fn new(c: Vector3) -> Self { SolidColor { color_value: c} }
    pub fn from_rgb(r: f64, g: f64, b: f64) -> Self { SolidColor::new(Vector3::new(r, g, b)) }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Vector3) -> Vector3 {
        self.color_value
    }
}

#[derive(Clone)]
pub struct CheckerTexture {
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>
}

impl CheckerTexture {
    pub fn new(even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        CheckerTexture { even, odd }
    }
    pub fn from_colors(c1: Vector3, c2: Vector3) -> Self {
        CheckerTexture {
            even: Arc::new(SolidColor::new(c1)),
            odd: Arc::new(SolidColor::new(c2))
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Vector3) -> Vector3 {
        let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}