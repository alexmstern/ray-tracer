use std::sync::Arc;

use ray_tracer::hittable::HittableList;
use ray_tracer::sphere::Sphere;
use ray_tracer::camera::Camera;
use ray_tracer::vector3::Vector3;
use ray_tracer::material::{Dielectric, Lambertian, Metal};

fn main() {
    let mut world = HittableList::new();

    let material_ground = Arc::new(Lambertian::new(Vector3::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Vector3::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.50));
    let material_bubble = Arc::new(Dielectric::new(1.00 / 1.50));
    let material_right = Arc::new(Metal::new(Vector3::new(0.8, 0.6, 0.2), 1.0));

    world.push(Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0, Some(material_ground)));
    world.push(Sphere::new(Vector3::new(0.0, 0.0, -1.2), 0.5, Some(material_center)));
    world.push(Sphere::new(Vector3::new(-1.0, 0.0, -1.0), 0.5, Some(material_left)));
    world.push(Sphere::new(Vector3::new(-1.0, 0.0, -1.0), 0.4, Some(material_bubble)));
    world.push(Sphere::new(Vector3::new(1.0, 0.0, -1.0), 0.5, Some(material_right)));

    let camera = Camera::new(16.0 / 9.0, 400, 100, 50);
    camera.render(&world);
}