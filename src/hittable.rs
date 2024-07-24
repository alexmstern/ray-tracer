use std::sync::Arc;

use crate::vector3::Vector3;
use crate::ray::Ray;
use crate::material::Material;

#[derive(Clone, Default)]
pub struct HitRecord {
    pub p: Vector3,
    pub normal: Vector3,
    pub mat: Option<Arc<dyn Material>>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vector3) {
        self.front_face = r.dir().dot(*outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else { -1.0 * *outward_normal };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

pub struct HittableList {
    list: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> Self { HittableList { list: Vec::new() } }
    pub fn push(&mut self, hittable: impl Hittable + 'static) {
        self.list.push(Box::new(hittable))
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut current_closest = t_max;

        for object in &self.list {
            let mut temp_rec = HitRecord::default();
            if object.hit(r, t_min, current_closest, &mut temp_rec) {
                hit_anything = true;
                current_closest = temp_rec.t;
                *rec = temp_rec;
            }
        }

        hit_anything
    }
}