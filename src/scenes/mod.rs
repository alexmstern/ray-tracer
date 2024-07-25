pub mod cover_scene;
pub mod depth_of_field_scene;

use crate::hittable::HittableList;
use crate::camera::Camera;

pub enum Scene {
    CoverScene,
    DepthOfFieldScene
}

pub fn get_scene(scene: Scene) -> (HittableList, Camera) {
    match scene {
        Scene::CoverScene => cover_scene::cover_scene(),
        Scene::DepthOfFieldScene => depth_of_field_scene::depth_of_field_scene()
    }
}