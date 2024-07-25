use ray_tracer::scenes;

fn main() {
    let scene = scenes::Scene::DepthOfFieldScene;
    let (world, camera) = scenes::get_scene(scene);
    camera.render(&world, "depth-of-field");
}