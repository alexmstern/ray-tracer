use image::{RgbImage, ImageBuffer, Rgb};
use indicatif::{ProgressBar, ProgressStyle};
use crate::hittable::{HitRecord, Hittable};
use crate::random_double;
use crate::vector3::Vector3;
use crate::ray::Ray;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub vfov: f64, // degrees
    pub look_from: Vector3,
    pub look_at: Vector3,
    pub v_up: Vector3,

    pub defocus_angle: f64,
    pub focus_dist: f64,

    pub time0: f64,
    pub time1: f64,

    pub image_height: u32,
    pub pixel_samples_scale: f64,
    pub pixel00_loc: Vector3,
    pub pixel_delta_u: Vector3,
    pub pixel_delta_v: Vector3,
    pub defocus_disk_u: Vector3,
    pub defocus_disk_v: Vector3,
}

impl Camera {

    pub fn new(aspect_ratio: f64, image_width: u32, samples_per_pixel: u32, max_depth: u32, vfov: f64, look_from: Vector3, look_at: Vector3, v_up: Vector3, defocus_angle: f64, focus_dist: f64, time0: f64, time1: f64) -> Self {
        let mut camera = Camera {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            vfov,
            look_from,
            look_at,
            v_up,
            defocus_angle,
            focus_dist,
            time0,
            time1,
            image_height: 0,
            pixel_samples_scale: 1.0,
            pixel00_loc: Vector3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vector3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vector3::new(0.0, 0.0, 0.0),
            defocus_disk_u: Vector3::new(0.0, 0.0, 0.0),
            defocus_disk_v: Vector3::new(0.0, 0.0, 0.0),
        };
        camera.initialize();
        camera
    }

    pub fn initialize(&mut self) {

        self.image_height = (self.image_width as f64 / self.aspect_ratio) as u32;
        self.image_height = if self.image_height < 1 { 1 } else { self.image_height };

        self.pixel_samples_scale = 1.0 / (self.samples_per_pixel as f64);

        // Determine viewport dimensions.
        let theta = 3.141592653589793 * self.vfov / 180.0;
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = (self.look_from - self.look_at).unit_vector();
        let u = self.v_up.cross(w).unit_vector();
        let v = w.cross(u);
    
        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * u;     // Vector across viewport horizontal edge
        let viewport_v = -viewport_height * v;   // Vector down viewport vertical edge
    
        // Calculate the horizontal and vertical delta vectors to the next pixel.
        self.pixel_delta_u = viewport_u / (self.image_width as f64);
        self.pixel_delta_v = viewport_v / (self.image_height as f64);
    
        // Calculate the location of the upper left pixel.
        let viewport_upper_left = self.look_from - (self.focus_dist * w) - (viewport_u / 2.0) - (viewport_v / 2.0);
        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = self.focus_dist * (3.141592653589793 * self.defocus_angle / 360.0).tan();
        self.defocus_disk_u = defocus_radius * u;
        self.defocus_disk_v = defocus_radius * v;
    }

    pub fn render(&self, world: &dyn Hittable, image_name: &str) {
        let mut buffer: RgbImage = ImageBuffer::new(self.image_width, self.image_height);
        let intro_message = format!("Rendering {}.png:", image_name);
        println!("{}", intro_message);
        let bar = ProgressBar::new((self.image_width * self.image_height) as u64);
        bar.set_style(ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:40.cyan/blue}]{pos:>7}/{len:7}\n{msg}\n")
            .expect("Unable to create progress bar style."));
        for (x, y, pixel) in buffer.enumerate_pixels_mut() {
            let mut pixel_color = Vector3::new(0.0, 0.0, 0.0);
            for _ in 0..self.samples_per_pixel {
                let ray = self.get_ray(x, y);
                pixel_color = pixel_color + ray_color(ray, self.max_depth, world);
            }
            pixel_color = self.pixel_samples_scale * pixel_color;
            let ir = (255.999 * pixel_color.x().sqrt()) as u8;
            let ig = (255.999 * pixel_color.y().sqrt()) as u8;
            let ib = (255.999 * pixel_color.z().sqrt()) as u8;

            *pixel = Rgb([ir, ig, ib]);

            bar.inc(1);
        }
        let finish_message = format!("Finished rendering {}.png!", image_name);
        bar.finish_with_message(finish_message);

        let image_path = format!("images/{}.png", image_name);
        buffer.save(image_path).unwrap();
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        // Construct a camera ray originating from the defocus disk and directed at a randomly
        // sampled point around the pixel location i, j.
        let pixel_sample = self.pixel00_loc
                         + ((i as f64 + random_double(-0.5, 0.5)) * self.pixel_delta_u)
                         + ((j as f64 + random_double(-0.5, 0.5)) * self.pixel_delta_v);
        let ray_origin = if self.defocus_angle <= 0.0 { self.look_from } else { self.defocus_disk_sample() };
        let ray_direction = pixel_sample - ray_origin;
        let time = if self.time0 == self.time1 { self.time0 } else { random_double(self.time0, self.time1) };
        Ray::new(ray_origin, ray_direction, time)
    }

    fn defocus_disk_sample(&self) -> Vector3 {
        let p = Vector3::random_in_unit_disk();
        self.look_from + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
    }

}

fn ray_color(r: Ray, depth: u32, world: &dyn Hittable) -> Vector3 {
    if depth <= 0 {
        return Vector3::new(0.0, 0.0, 0.0);
    }
    
    let mut rec = HitRecord::default();
    if world.hit(&r, 0.001, f64::INFINITY, &mut rec) {

        let mut scattered = Ray::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0), r.time());
        let mut attenuation = Vector3::new(0.0, 0.0, 0.0);
        
        if let Some(mat) = &rec.mat {
            if mat.scatter(&r, &rec, &mut attenuation, &mut scattered) {
                return attenuation * ray_color(scattered, depth-1, world);
            }
        }
        return Vector3::new(0.0, 0.0, 0.0);
    }

    let unit_direction = r.dir().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    return Vector3::new(1.0,1.0,1.0) * (1.0 - a) + Vector3::new(0.5,0.7,1.0) * a;
}