pub mod cover_scene;
pub mod depth_of_field_scene;
pub mod bouncing_spheres_scene;

use crate::hittable::HittableList;
use crate::camera::Camera;

pub enum Scene {
    CoverScene,
    DepthOfFieldScene,
    BouncingSpheresScene
}

pub fn get_scene(scene: Scene) -> (HittableList, Camera) {
    match scene {
        Scene::CoverScene => cover_scene::cover_scene(),
        Scene::DepthOfFieldScene => depth_of_field_scene::depth_of_field_scene(),
        Scene::BouncingSpheresScene => bouncing_spheres_scene::bouncing_spheres_scene(),
    }
}