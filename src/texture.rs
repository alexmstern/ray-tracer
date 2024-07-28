use std::sync::Arc;
use image::RgbImage;


use crate::perlin::Perlin;
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

pub struct ImageTexture {
    image: RgbImage
}

impl ImageTexture {
    pub fn new(image_name: &str) -> Self {
        let image_path = format!("images/{}", image_name);
        let image = image::open(image_path).expect("Failed to open image").to_rgb8();
        Self { image }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Vector3) -> Vector3 {
        let (width, height) = self.image.dimensions();
        let i = (u * width as f64) as u32;
        let j = ((1.0 - v) * height as f64 - 0.001) as u32;
        let i = i.min(width - 1);
        let j = j.min(height - 1);

        let pixel = self.image.get_pixel(i, j);
        let r = pixel[0] as f64 / 255.0;
        let g = pixel[1] as f64 / 255.0;
        let b = pixel[2] as f64 / 255.0;

        Vector3::new(r*r,g*g,b*b)
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self { NoiseTexture { noise: Perlin::new(), scale } }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Vector3) -> Vector3 {
        Vector3::new(0.5, 0.5, 0.5) * (1.0 + (self.scale * p.z() + 10.0 * self.noise.turb(p, 7)).sin())
    }
}