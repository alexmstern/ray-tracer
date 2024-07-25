use ray_tracer::scenes;

fn main() {
    let scene = scenes::Scene::BouncingSpheresScene;
    let (world, camera) = scenes::get_scene(scene);
    camera.render(&world, "bouncing-spheres");
}