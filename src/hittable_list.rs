use crate::{
    hittable::{Hit, HitRecord},
    interval::Interval,
    ray::Ray,
};

pub type HittableList = Vec<Box<dyn Hit>>;

impl Hit for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut temp_rec = None;
        let mut closest_so_far = ray_t.max;

        for object in self {
            if let Some(rec) =
                object.hit(r, Interval::new(ray_t.min, closest_so_far))
            {
                closest_so_far = rec.t;
                temp_rec = Some(rec);
            }
        }

        temp_rec
    }
}
