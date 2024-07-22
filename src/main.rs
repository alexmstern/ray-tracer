mod vector3;
mod color;
mod ray;

use std::io;

use crate::vector3::Vector3;
use crate::color::Color;
use crate::color::write_color;
use crate::ray::Ray;

fn ray_color(r: Ray<f64>) -> Color {
    let unit_direction = r.dir.unit_vector();
    let a = 0.5 * (unit_direction.y + 1.0);
    return Color::new(1.0,1.0,1.0) * (1.0 - a) + Color::new(0.5,0.7,1.0) * a;
}

fn main() {

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    let mut image_height = (image_width as f64 / aspect_ratio).round() as i32;
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


    let stdout = io::stdout();
    let mut handle = stdout.lock();

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center = pixel00_loc + (pixel_delta_u * (i as f64)) + (pixel_delta_v * (j as f64));
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(pixel_center, ray_direction);
            
            let pixel_color = ray_color(r);
            write_color(&mut handle, &pixel_color);
        }
    }
}
