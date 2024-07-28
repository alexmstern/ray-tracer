use ray_tracer::scenes;

fn main() {
    let scene = scenes::Scene::PerlinScene;
    let (world, camera) = scenes::get_scene(scene);
    camera.render(&world, "perlin");
}