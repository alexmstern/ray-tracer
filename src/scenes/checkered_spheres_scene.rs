use std::sync::Arc;
use crate::hittable::HittableList;
use crate::sphere::Sphere;
use crate::camera::Camera;
use crate::vector3::Vector3;
use crate::material::Lambertian;
use crate::texture::CheckerTexture;

pub fn checkered_spheres_scene() -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let checker_texture = Arc::new(CheckerTexture::from_colors(Vector3::new(0.2, 0.3, 0.1), Vector3::new(0.9, 0.9, 0.9)));
    let checker_material = Arc::new(Lambertian::new_from_texture(checker_texture));
    world.push(Sphere::new(Vector3::new(0.0, -10.0, 0.0), 10.0, Some(checker_material.clone())));
    world.push(Sphere::new(Vector3::new(0.0, 10.0, 0.0), 10.0, Some(checker_material.clone())));

    let camera = Camera::new(16.0 / 9.0, 400, 500, 50, 20.0, Vector3::new(13.0, 2.0, 3.0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), 0.0, 10.0, 0.0, 0.0);
    
    (world, camera)
}