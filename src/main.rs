use ray_tracer::scenes;

fn main() {
    let scene = scenes::Scene::CheckeredSpheresScene;
    let (world, camera) = scenes::get_scene(scene);
    camera.render(&world, "checkered-spheres");
}