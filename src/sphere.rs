use std::f64;

use crate::vector3::Vector3;
use crate::ray::Ray;
use crate::hittable::{HitRecord, Hittable};

pub struct Sphere {
    center: Vector3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f64) -> Sphere { Sphere { center, radius: radius.max(0.0) } }
    pub fn center(&self) -> Vector3 { self.center }
    pub fn radius(&self) -> f64 { self.radius }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = self.center() - r.orig();
        let a = r.dir().dot(r.dir());
        let h = r.dir().dot(oc);
        let c = oc.dot(oc) - self.radius() * self.radius();
        let discriminant = h*h - a*c;

        if discriminant < 0.0 { 
            return false;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;
        if root <= t_min || t_max <= root {
            root = (h + sqrtd) / a;
            if root <= t_min || t_max <= root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center()) / self.radius();
        rec.set_face_normal(r, &outward_normal);

        true
    }
}