use super::Point;
use super::Vector;

pub struct Ray {
    pub origin: Point,
    pub dir: Vector,
}

impl Ray {
    pub fn new(origin: Point, dir: Vector) -> Self {
        Ray { origin, dir }
    }

    pub fn at(&self, t: f32) -> Point {
        self.origin + t * self.dir
    }
}
