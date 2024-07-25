use std::sync::Arc;
use crate::hittable::HittableList;
use crate::random_double;
use crate::sphere::Sphere;
use crate::camera::Camera;
use crate::vector3::Vector3;
use crate::material::{Dielectric, Lambertian, Metal};

pub fn cover_scene() -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(Vector3::new(0.5, 0.5, 0.5)));
    world.push(Sphere::new(Vector3::new(0.0, -1000.0, 0.0), 1000.0, Some(ground_material)));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double(0.0, 1.0);
            let center = Vector3::new((a as f64) + random_double(0.0, 0.9), 0.2, (b as f64) + random_double(0.0, 0.9));

            if (center - Vector3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Vector3::random(0.0, 1.0) * Vector3::random(0.0, 1.0);
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    world.push(Sphere::new(center, 0.2, Some(sphere_material)));
                } else if choose_mat < 0.95 {
                    let albedo = Vector3::random(0.5, 1.0);
                    let fuzz = random_double(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.push(Sphere::new(center, 0.2, Some(sphere_material)));
                } else {
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.push(Sphere::new(center, 0.2, Some(sphere_material)));

                }


            }
        } 
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.push(Sphere::new(Vector3::new(0.0, 1.0, 0.0), 1.0, Some(material1)));
    let material2 = Arc::new(Lambertian::new(Vector3::new(0.4, 0.2, 0.1)));
    world.push(Sphere::new(Vector3::new(-4.0, 1.0, 0.0), 1.0, Some(material2)));
    let material3 = Arc::new(Metal::new(Vector3::new(0.7, 0.6, 0.5), 0.0));
    world.push(Sphere::new(Vector3::new(4.0, 1.0, 0.0), 1.0, Some(material3)));

    let camera = Camera::new(16.0 / 9.0, 1200, 500, 50, 20.0, Vector3::new(13.0, 2.0, 3.0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), 0.6, 10.0);
    
    (world, camera)
}