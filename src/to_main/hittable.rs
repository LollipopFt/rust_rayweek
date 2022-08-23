use super::{ray::Ray, Point, Vector};

pub struct HitRecord {
    pub p: Point,
    pub normal: Vector,
    pub t: f32,
}

pub trait Hit {
    fn hit(&self, r: &Ray, ray_tmin: f32, ray_tmax: f32) -> Option<HitRecord>;
}
