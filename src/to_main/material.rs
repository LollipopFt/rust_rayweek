use rand::Rng;

use super::{
    hittable::HitRecord,
    ray::Ray,
    vec3::{random_in_unit_sphere, random_unit_vector},
    Color,
};
use crate::to_main::vec3::Extensions;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(color: Color) -> Self {
        Lambertian { albedo: color }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_dir = rec.normal + random_unit_vector();
        // intercept zero scatter condition
        if scatter_dir.near_zero() {
            scatter_dir = rec.normal;
        }
        let scattered = Ray::new(rec.p, scatter_dir);
        Some((self.albedo, scattered))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(color: Color, fuzz: f32) -> Self {
        if fuzz <= 1. {
            Metal { albedo: color, fuzz }
        } else {
            Metal { albedo: color, fuzz: 1. }
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = r_in.dir.normalize().reflect(&rec.normal);
        let scattered =
            Ray::new(rec.p, reflected + self.fuzz * random_in_unit_sphere());
        if scattered.dir.dot(&rec.normal) > 0. {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    ir: f32,
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let refraction_ratio =
            if rec.front_face { 1. / self.ir } else { self.ir };

        let unit_dir = r_in.dir.normalize();
        let cos_theta = (-unit_dir).dot(&rec.normal).min(1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.;
        let direction = if cannot_refract
            || reflectance(cos_theta, refraction_ratio)
                > rand::thread_rng().gen::<f32>()
        {
            unit_dir.reflect(&rec.normal)
        } else {
            unit_dir.refract(&rec.normal, refraction_ratio)
        };

        Some((Color::new(1., 1., 1.), Ray::new(rec.p, direction)))
    }
}

impl Dielectric {
    pub fn new(ir: f32) -> Self {
        Dielectric { ir }
    }
}

fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1. - ref_idx) / (1. + ref_idx);
    let r0 = r0 * r0;
    r0 + (1. - r0) * (1. - cosine).powi(5)
}
