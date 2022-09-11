use std::rc::Rc;

use crate::{interval::Interval, material::Material, ray::Ray, Point, Vector};

pub struct HitRecord {
    pub p: Point,
    pub normal: Vector,
    pub mat: Rc<dyn Material>,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vector) {
        // normals always point against the incident ray
        self.front_face = r.dir.dot(&outward_normal) < 0.;
        self.normal =
            if self.front_face { outward_normal } else { -outward_normal };
    }
}

pub trait Hit {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}
