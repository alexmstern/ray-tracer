use std::sync::Arc;
use crate::random_double;
use crate::vector3::Vector3;
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::texture::{Texture, SolidColor};

pub trait Material {
    fn scatter(&self, r: &Ray, rec: &HitRecord, attenuation: &mut Vector3, scattered: &mut Ray) -> bool;
}

pub struct Lambertian {
    albedo: Arc<dyn Texture>
}

impl Lambertian {
    pub fn new(albedo: Vector3) -> Self {
        let solid_color_texture = Arc::new(SolidColor::new(albedo));
        Lambertian { albedo: solid_color_texture }
    }
    pub fn new_from_texture(albedo: Arc<dyn Texture>) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r: &Ray, rec: &HitRecord, attenuation: &mut Vector3, scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal + Vector3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *scattered = Ray::new(rec.p, scatter_direction, r.time());
        *attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        true
    }
}

pub struct Metal {
    albedo: Vector3,
    fuzz: f64
}

impl Metal {
    pub fn new(albedo: Vector3, fuzz: f64) -> Self { Metal { albedo, fuzz: if fuzz < 1.0 {fuzz} else {1.0} } }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, rec: &HitRecord, attenuation: &mut Vector3, scattered: &mut Ray) -> bool {
        let mut reflected = r.dir().reflect(rec.normal);
        reflected = reflected.unit_vector() + (self.fuzz * Vector3::random_unit_vector());
        *scattered = Ray::new(rec.p, reflected, r.time());
        *attenuation = self.albedo;
        scattered.dir().dot(rec.normal) > 0.0
    }
}

pub struct Dielectric {
    refraction_index: f64
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self { Dielectric { refraction_index } }

    fn reflectance(&self, cosine: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1.0 - self.refraction_index) / (1.0 + self.refraction_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r: &Ray, rec: &HitRecord, attenuation: &mut Vector3, scattered: &mut Ray) -> bool {
        *attenuation = Vector3::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face { 1.0 / self.refraction_index } else { self.refraction_index };
        let unit_direction = r.dir().unit_vector();

        let cos_theta = f64::min(-1.0 * unit_direction.dot(rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let direction;
        if cannot_refract || self.reflectance(cos_theta) > random_double(0.0, 1.0) {
            direction = unit_direction.reflect(rec.normal);
        } else {
            direction = unit_direction.refract(rec.normal, ri);
        }

        *scattered = Ray::new(rec.p, direction, r.time());
        true
    }
}