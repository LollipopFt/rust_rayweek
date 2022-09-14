use crate::{ray::Ray, Point, Vector};

pub struct Camera {
    origin: Point,
    lower_left: Point,
    horizontal: Vector,
    vertical: Vector,
}

impl Camera {
    pub fn init(&mut self, aspect_ratio: f32) {
        let viewport_height = 2.;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.;

        self.origin = Point::new(0., 0., 0.);
        self.horizontal = Vector::new(viewport_width, 0., 0.);
        self.vertical = Vector::new(0., viewport_height, 0.);
        self.lower_left = self.origin
            - self.horizontal / 2.
            - self.vertical / 2.
            - Vector::new(0., 0., focal_length);
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left + s * self.horizontal + (1. - t) * self.vertical
                - self.origin,
        )
    }
}

impl Default for Camera {
    fn default() -> Self {
        const ASPECT_RATIO: f32 = 16. / 9.;
        const VIEWPORT_HEIGHT: f32 = 2.;
        const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;
        const FOCAL_LENGTH: f32 = 1.;

        let origin = Point::new(0., 0., 0.);
        let horizontal = Vector::new(VIEWPORT_WIDTH, 0., 0.);
        let vertical = Vector::new(0., VIEWPORT_HEIGHT, 0.);
        let lower_left = origin
            - horizontal / 2.
            - vertical / 2.
            - Vector::new(0., 0., FOCAL_LENGTH);
        Camera { origin, lower_left, horizontal, vertical }
    }
}
