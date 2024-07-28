pub mod cover_scene;
pub mod depth_of_field_scene;
pub mod bouncing_spheres_scene;
pub mod checkered_spheres_scene;
pub mod earth_scene;
pub mod perlin_scene;

use crate::hittable::HittableList;
use crate::camera::Camera;

pub enum Scene {
    CoverScene,
    DepthOfFieldScene,
    BouncingSpheresScene,
    CheckeredSpheresScene,
    EarthScene,
    PerlinScene
}

pub fn get_scene(scene: Scene) -> (HittableList, Camera) {
    match scene {
        Scene::CoverScene => cover_scene::cover_scene(),
        Scene::DepthOfFieldScene => depth_of_field_scene::depth_of_field_scene(),
        Scene::BouncingSpheresScene => bouncing_spheres_scene::bouncing_spheres_scene(),
        Scene::CheckeredSpheresScene => checkered_spheres_scene::checkered_spheres_scene(),
        Scene::EarthScene => earth_scene::earth_scene(),
        Scene::PerlinScene => perlin_scene::perlin_scene()
    }
}