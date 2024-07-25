use std::f64;
use std::sync::Arc;

use crate::vector3::Vector3;
use crate::ray::Ray;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;

pub struct MovingSphere {
    center0: Vector3,
    center1: Vector3,
    time0: f64,
    time1: f64,
    radius: f64,
    mat: Option<Arc<dyn Material>>,
}

impl MovingSphere {
    pub fn new(center0: Vector3, center1: Vector3, time0: f64, time1: f64, radius: f64, mat: Option<Arc<dyn Material>>) -> MovingSphere {
         MovingSphere { center0, center1, time0, time1, radius: radius.max(0.0), mat } 
    }
    pub fn center(&self, time: f64) -> Vector3 {
        self.center0 + ((time - self.time0)/(self.time1 - self.time0)) * (self.center1 - self.center0)
    }
    pub fn radius(&self) -> f64 { self.radius }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = self.center(r.time()) - r.orig();
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
        let outward_normal = (rec.p - self.center(r.time())) / self.radius();
        rec.set_face_normal(r, &outward_normal);
        rec.mat = self.mat.clone();

        true
    }
}