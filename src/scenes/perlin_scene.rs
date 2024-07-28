use std::sync::Arc;
use crate::hittable::HittableList;
use crate::sphere::Sphere;
use crate::camera::Camera;
use crate::vector3::Vector3;
use crate::material::Lambertian;
use crate::texture::NoiseTexture;

pub fn perlin_scene() -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let perlin_texture = Arc::new(NoiseTexture::new(4.0));
    let perlin_material = Arc::new(Lambertian::new_from_texture(perlin_texture));
    world.push(Sphere::new(Vector3::new(0.0, -1000.0, 0.0), 1000.0, Some(perlin_material.clone())));
    world.push(Sphere::new(Vector3::new(0.0, 2.0, 0.0), 2.0, Some(perlin_material.clone())));

    let camera = Camera::new(16.0 / 9.0, 400, 100, 50, 20.0, Vector3::new(13.0, 2.0, 3.0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), 0.0, 10.0, 0.0, 0.0);
    
    (world, camera)
}