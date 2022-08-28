use nalgebra::Vector3;
use rand::Rng;

trait Random {
    fn random() -> Self;
    fn rand(min: f32, max: f32) -> Self;
}

impl Random for Vector3<f32> {
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
