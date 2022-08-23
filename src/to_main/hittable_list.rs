use super::{
    hittable::{Hit, HitRecord},
    ray::Ray,
};

pub type HittableList = Vec<Box<dyn Hit>>;

impl Hit for HittableList {
    fn hit(&self, r: &Ray, ray_tmin: f32, ray_tmax: f32) -> Option<HitRecord> {
        let mut temp_rec = None;
        let mut closest_so_far = ray_tmax;

        for object in self {
            if let Some(rec) = object.hit(r, ray_tmin, closest_so_far) {
                closest_so_far = rec.t;
                temp_rec = Some(rec);
            }
        }

        temp_rec
    }
}
