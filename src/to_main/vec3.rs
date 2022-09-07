use nalgebra::Vector3;
use rand::Rng;

use super::Vector;

pub trait Extensions {
    fn random() -> Self;
    fn rand(min: f32, max: f32) -> Self;
    fn near_zero(&self) -> bool;
    fn refract(&self, n: &Vector, etai_over_etat: f32) -> Self;
}

impl Extensions for Vector3<f32> {
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        Vector3::new(rng.gen(), rng.gen(), rng.gen())
    }

    fn rand(min: f32, max: f32) -> Self {
        let mut rng = rand::thread_rng();
        Vector3::new(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }

    fn near_zero(&self) -> bool {
        let s = 1e-8;
        self[0].abs() < s && self[1].abs() < s && self[2].abs() < s
    }

    fn refract(&self, n: &Vector, etai_over_etat: f32) -> Self {
        let cos_theta = (-self).dot(n).min(1.);
        let r_out_perp = etai_over_etat * (self + cos_theta * n);
        let r_out_parallel =
            -((1. - r_out_perp.norm_squared()).abs().sqrt()) * n;
        r_out_perp + r_out_parallel
    }
}

pub fn random_in_unit_sphere() -> Vector3<f32> {
    loop {
        let p = Vector3::rand(-1., 1.);
        if p.norm_squared() >= 1. {
            continue;
        }
        return p;
    }
}

pub fn random_unit_vector() -> Vector3<f32> {
    random_in_unit_sphere().normalize()
}

pub fn reflect(v: &Vector, n: &Vector) -> Vector {
    v - 2. * v.dot(n) * n
}
