use std::sync::Arc;
use crate::hittable::HittableList;
use crate::sphere::Sphere;
use crate::camera::Camera;
use crate::vector3::Vector3;
use crate::material::{Dielectric, Lambertian, Metal};

pub fn depth_of_field_scene() -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let material_ground = Arc::new(Lambertian::new(Vector3::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Vector3::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_bubble = Arc::new(Dielectric::new(1.0 / 1.5));
    let material_right = Arc::new(Metal::new(Vector3::new(0.8, 0.6, 0.2), 1.0));

    world.push(Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0, Some(material_ground)));
    world.push(Sphere::new(Vector3::new(0.0, 0.0, -1.2), 0.5, Some(material_center)));
    world.push(Sphere::new(Vector3::new(-1.0, 0.0, -1.0), 0.5, Some(material_left)));
    world.push(Sphere::new(Vector3::new(-1.0, 0.0, -1.0), 0.4, Some(material_bubble)));
    world.push(Sphere::new(Vector3::new(1.0, 0.0, -1.0), 0.5, Some(material_right)));

    let camera = Camera::new(16.0 / 9.0, 400, 100, 50, 20.0, Vector3::new(-2.0, 2.0, 1.0), Vector3::new(0.0, 0.0, -1.0), Vector3::new(0.0, 1.0, 0.0), 10.0, 3.4);
    
    (world, camera)
}