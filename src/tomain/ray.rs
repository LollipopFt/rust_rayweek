use super::Vector;
use super::Point;

pub struct Ray {
    pub origin: Point,
    pub dir: Vector
}

impl Ray {
    fn new(origin: Point, direction: Vector) -> Self {
        Ray {
            origin,
            dir: direction
        }
    }

    fn at(&self, t: f32) -> Point {
        self.origin + t * self.dir
    }
}
