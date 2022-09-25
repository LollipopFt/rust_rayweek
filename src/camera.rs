use crate::{ray::Ray, vec3::random_in_unit_disk, Point, Vector};

pub struct Camera {
    origin: Point,
    lower_left: Point,
    horizontal: Vector,
    vertical: Vector,
    u: Vector,
    v: Vector,
    w: Vector,
    lens_radius: f32,

    pub vfov: f32,
    pub aperture: f32,
    pub focus_dist: f32,

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

        self.w = (self.lookfrom - self.lookat).normalize();
        self.u = self.vup.cross(&self.w).normalize();
        self.v = self.w.cross(&self.u);

        self.origin = self.lookfrom;
        self.horizontal = self.focus_dist * viewport_width * self.u;
        self.vertical = self.focus_dist * viewport_height * self.v;
        self.lower_left = self.origin
            - self.horizontal / 2.
            - self.vertical / 2.
            - self.focus_dist * self.w;

        self.lens_radius = self.aperture / 2.;
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        Ray::new(
            self.origin + offset,
            self.lower_left + s * self.horizontal + (1. - t) * self.vertical
                - self.origin
                - offset,
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
            u: Default::default(),
            v: Default::default(),
            w: Default::default(),
            lens_radius: Default::default(),

            vfov: 40.,
            aperture: 0.,
            focus_dist: 10.,

            lookfrom: Point::new(0., 0., -1.),
            lookat: Point::new(0., 0., 0.),
            vup: Vector::new(0., 1., 0.),
        }
    }
}
