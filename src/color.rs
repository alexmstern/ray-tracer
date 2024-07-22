use std::io::Write;
use crate::vector3::Vector3;

pub type Color = Vector3<f64>;

pub fn write_color(out: &mut dyn Write, pixel_color: &Color) {
    let r = pixel_color.x;
    let g = pixel_color.y;
    let b = pixel_color.z;
    let rbyte = (255.999 * r).round() as u8;
    let gbyte = (255.999 * g).round() as u8;
    let bbyte = (255.999 * b).round() as u8;

    writeln!(out, "{} {} {}", rbyte, gbyte, bbyte).expect("Failed to write to output");
}