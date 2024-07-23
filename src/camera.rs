use image::{RgbImage, ImageBuffer, Rgb};
use crate::hittable::{HitRecord, Hittable};
use crate::vector3::Vector3;
use crate::ray::Ray;
use rand::Rng;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub samples_per_pixel: u32,

    pub image_height: u32,
    pub pixel_samples_scale: f64,
    pub center: Vector3,
    pub pixel00_loc: Vector3,
    pub pixel_delta_u: Vector3,
    pub pixel_delta_v: Vector3,
}

impl Camera {

    pub fn new(aspect_ratio: f64, image_width: u32, samples_per_pixel: u32) -> Self {
        let mut camera = Camera {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            image_height: 0,
            pixel_samples_scale: 1.0,
            center: Vector3::new(0.0, 0.0, 0.0),
            pixel00_loc: Vector3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vector3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vector3::new(0.0, 0.0, 0.0),
        };
        camera.initialize();
        camera
    }

    pub fn initialize(&mut self) {

        self.image_height = (self.image_width as f64 / self.aspect_ratio) as u32;
        self.image_height = if self.image_height < 1 { 1 } else { self.image_height };

        self.pixel_samples_scale = 1.0 / (self.samples_per_pixel as f64);

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);
    
        let viewport_u = Vector3::new(viewport_width,0.0,0.0);
        let viewport_v = Vector3::new(0.0,-viewport_height,0.0);
    
        self.pixel_delta_u = viewport_u / (self.image_width as f64);
        self.pixel_delta_v = viewport_v / (self.image_height as f64);
    
        let viewport_upper_left = self.center - Vector3::new(0.0,0.0,focal_length) - (viewport_u / 2.0) - (viewport_v / 2.0);
        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;
    }

    pub fn render(&self, world: &dyn Hittable) {
        let mut buffer: RgbImage = ImageBuffer::new(self.image_width, self.image_height);
        for (x, y, pixel) in buffer.enumerate_pixels_mut() {
            let mut pixel_color = Vector3::new(0.0, 0.0, 0.0);
            for _ in 0..self.samples_per_pixel {
                let ray = self.get_ray(x, y);
                pixel_color = pixel_color + ray_color(ray, world);
            }
            pixel_color = self.pixel_samples_scale * pixel_color;
            let ir = (255.999 * pixel_color.x()) as u8;
            let ig = (255.999 * pixel_color.y()) as u8;
            let ib = (255.999 * pixel_color.z()) as u8;

            *pixel = Rgb([ir, ig, ib]);
        }
        buffer.save("output/image.png").unwrap();
    }

    pub fn get_ray(&self, i: u32, j: u32) -> Ray {
        let mut rng = rand::thread_rng();
        let pixel_sample = self.pixel00_loc 
                                  + ((i as f64 + rng.gen_range(-0.5..0.5)) * self.pixel_delta_u)
                                  + ((j as f64 + rng.gen_range(-0.5..0.5)) * self.pixel_delta_v);
        Ray::new(self.center, pixel_sample - self.center)
    }

}

fn ray_color(r: Ray, world: &dyn Hittable) -> Vector3 {
    let mut rec = HitRecord::default();
    if world.hit(&r, 0.0, f64::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Vector3::new(1.0, 1.0, 1.0));
    }

    let unit_direction = r.dir().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    return Vector3::new(1.0,1.0,1.0) * (1.0 - a) + Vector3::new(0.5,0.7,1.0) * a;
}