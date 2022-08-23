use super::{
    hittable::{Hit, HitRecord},
    Point,
};

struct Sphere {
    ctr: Point,
    r: f32,
}

impl Sphere {
    fn new(ctr: Point, r: f32) -> Self {
        Sphere { ctr, r }
    }
}

impl Hit for Sphere {
    fn hit(
        &self,
        r: &super::ray::Ray,
        ray_tmin: f32,
        ray_tmax: f32,
    ) -> Option<HitRecord> {
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
        if root < ray_tmin || ray_tmax < root {
            root = (-half_b + sqrtd) / a;
            if root < ray_tmin || ray_tmax < root {
                return None;
            }
        }

        let p = r.at(root);
        Some(HitRecord { t: root, p, normal: (p - self.ctr) / self.r })
    }
}
