use std::rc::Rc;

use super::{
    hittable::{Hit, HitRecord},
    interval::Interval,
    material::Material,
    ray::Ray,
    Point, Vector,
};

pub struct Sphere {
    pub ctr: Point,
    pub r: f32,
    pub mat: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(ctr: Point, r: f32, mat: Rc<dyn Material>) -> Self {
        Sphere { ctr, r, mat }
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        // ray equation: P(t) = A + tb
        // in a sphere: (P(t)-C)∙(P(t)-C) = r² => (A+tb-C)∙(a+tb-C) = r²
        // t²b∙b + 2tb∙(A-C) + (A-C)∙(A-C) = r²
        // a: b∙b, b: 2b∙(A-C), c: (A-C)∙(A-C) - r²
        let oc = r.origin - self.ctr;
        let a = r.dir.norm_squared();
        let half_b = oc.dot(&r.dir);
        let c = oc.norm_squared() - self.r * self.r;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.contains(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.contains(root) {
                return None;
            }
        }

        let mut rec = HitRecord {
            p: r.at(root),
            t: root,
            mat: self.mat.clone(),
            normal: Vector::default(),
            front_face: bool::default(),
        };
        let outward_normal = (rec.p - self.ctr) / self.r;
        rec.set_face_normal(r, outward_normal);
        Some(rec)
    }
}
