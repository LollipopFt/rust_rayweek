use super::{hittable::HitRecord, ray::Ray, vec3::random_unit_vector, Color};
use crate::to_main::vec3::Extensions;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_dir = rec.normal + random_unit_vector();
        // intercept zero scatter condition
        if scatter_dir.near_zero() {
            scatter_dir = rec.normal;
        }
        let scattered = Ray::new(rec.p, scatter_dir);
        Some((self.albedo, scattered))
    }
}
