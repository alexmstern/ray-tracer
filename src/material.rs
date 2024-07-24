use crate::vector3::Vector3;
use crate::ray::Ray;
use crate::hittable::HitRecord;

pub trait Material {
    fn scatter(&self, r: &Ray, rec: &HitRecord, attenuation: &mut Vector3, scattered: &mut Ray) -> bool;
}

pub struct Lambertian {
    albedo: Vector3
}

impl Lambertian {
    pub fn new(albedo: Vector3) -> Self { Lambertian { albedo } }
}

impl Material for Lambertian {
    fn scatter(&self, _r: &Ray, rec: &HitRecord, attenuation: &mut Vector3, scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal + Vector3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
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
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        true
    }
}