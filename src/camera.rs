use crate::{ray::Ray, Point, Vector};

pub struct Camera {
    origin: Point,
    lower_left: Point,
    horizontal: Vector,
    vertical: Vector,
    pub vfov: f32,
    pub lookfrom: Point,
    pub lookat: Point,
    pub vup: Vector,
}

impl Camera {
    pub fn init(&mut self, aspect_ratio: f32) {
        let theta = self.vfov.to_radians();
        let h = (theta / 2.).tan();
        let viewport_height = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (self.lookfrom - self.lookat).normalize();
        let u = self.vup.cross(&w).normalize();
        let v = w.cross(&u);

        self.origin = self.lookfrom;
        self.horizontal = viewport_width * u;
        self.vertical = viewport_height * v;
        self.lower_left =
            self.origin - self.horizontal / 2. - self.vertical / 2. - w;
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
        let aspect_ratio: f32 = 16. / 9.;
        let viewport_height: f32 = 2.;
        let viewport_width: f32 = aspect_ratio * viewport_height;
        let focal_length: f32 = 1.;

        let origin = Point::new(0., 0., 0.);
        let horizontal = Vector::new(viewport_width, 0., 0.);
        let vertical = Vector::new(0., viewport_height, 0.);
        let lower_left = origin
            - horizontal / 2.
            - vertical / 2.
            - Vector::new(0., 0., focal_length);
        Camera {
            origin,
            lower_left,
            horizontal,
            vertical,
            vfov: 40.,
            lookfrom: Point::new(0., 0., -1.),
            lookat: Point::new(0., 0., 0.),
            vup: Vector::new(0., 1., 0.),
        }
    }
}
