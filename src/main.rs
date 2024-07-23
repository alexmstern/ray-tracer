mod vector3;
mod ray;

use image::{RgbImage, ImageBuffer, Rgb};

use crate::vector3::Vector3;
use crate::ray::Ray;

fn hit_sphere(center: Vector3, radius: f64, r: Ray) -> f64 {
    let oc = center - r.orig();
    let a = r.dir().dot(r.dir());
    let h = r.dir().dot(oc);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = h*h - a*c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (h - discriminant.sqrt()) / a;
    }
}

fn ray_color(r: Ray) -> Vector3 {
    let t = hit_sphere(Vector3::new(0.0,0.0,-1.0), 0.5, r);
    if t > 0.0 {
        let n = (r.at(t) - Vector3::new(0.0,0.0,-1.0)).unit_vector();
        return Vector3::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0) * 0.5;

    }
    let unit_direction = r.dir().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    return Vector3::new(1.0,1.0,1.0) * (1.0 - a) + Vector3::new(0.5,0.7,1.0) * a;
}

fn main() {

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    let mut image_height = (image_width as f64 / aspect_ratio).round() as u32;
    image_height = if image_height < 1 { 1 } else { image_height };
    
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Vector3::new(0.0,0.0,0.0);

    let viewport_u = Vector3::new(viewport_width,0.0,0.0);
    let viewport_v = Vector3::new(0.0,-viewport_height,0.0);

    let pixel_delta_u = viewport_u / (image_width as f64);
    let pixel_delta_v = viewport_v / (image_height as f64);

    let viewport_upper_left = camera_center - Vector3::new(0.0,0.0,focal_length) - (viewport_u / 2.0) - (viewport_v / 2.0);
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    let mut buffer: RgbImage = ImageBuffer::new(image_width, image_height);
    for (x, y, pixel) in buffer.enumerate_pixels_mut(){
        
        let pixel_center = pixel00_loc + (pixel_delta_u * (x as f64)) + (pixel_delta_v * (y as f64));
        let ray_direction = pixel_center - camera_center;
        let r = Ray::new(camera_center, ray_direction);
        let pixel_color = ray_color(r);
        
        let ir = (255.999 * pixel_color.x()) as u8;
        let ig = (255.999 * pixel_color.y()) as u8;
        let ib = (255.999 * pixel_color.z()) as u8;

        *pixel = Rgb([ir, ig, ib]);
    }

    buffer.save("output/normal_sphere.png").unwrap();

}