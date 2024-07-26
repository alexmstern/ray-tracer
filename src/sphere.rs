use std::f64;
use std::f64::consts::PI;
use std::sync::Arc;

use crate::vector3::Vector3;
use crate::ray::Ray;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;

pub struct Sphere {
    center: Vector3,
    radius: f64,
    mat: Option<Arc<dyn Material>>,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f64, mat: Option<Arc<dyn Material>>) -> Sphere { Sphere { center, radius: radius.max(0.0), mat } }
    pub fn center(&self) -> Vector3 { self.center }
    pub fn radius(&self) -> f64 { self.radius }
    pub fn get_sphere_uv(&self, p: Vector3) -> (f64, f64) {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;

        let u = phi / (2.0 * PI);
        let v = theta / PI;
        (u, v)
    }
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
        (rec.u, rec.v) = self.get_sphere_uv(outward_normal);
        rec.mat = self.mat.clone();

        true
    }
}