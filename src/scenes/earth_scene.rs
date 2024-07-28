use std::sync::Arc;
use crate::hittable::HittableList;
use crate::sphere::Sphere;
use crate::camera::Camera;
use crate::vector3::Vector3;
use crate::material::Lambertian;
use crate::texture::ImageTexture;

pub fn checkered_spheres_scene() -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let earth_texture = Arc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian::new_from_texture(earth_texture));
    world.push(Sphere::new(Vector3::new(0.0, 0.0, 0.0), 2.0, Some(earth_surface.clone())));

    let camera = Camera::new(16.0 / 9.0, 400, 100, 50, 20.0, Vector3::new(0.0, 0.0, 12.0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), 0.0, 10.0, 0.0, 0.0);
    
    (world, camera)
}