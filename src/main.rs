use ray_tracer::hittable::HittableList;
use ray_tracer::sphere::Sphere;
use ray_tracer::camera::Camera;
use ray_tracer::vector3::Vector3;

fn main() {
    let mut world = HittableList::new();
    world.push(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5));
    world.push(Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0));

    let camera = Camera::new(16.0 / 9.0, 400, 100);
    camera.render(&world);
}