use crate::{
    camera::Camera, hittable::Hit, hittable_list::HittableList, writecolor,
    Color, Interval, Ray,
};
use rand::Rng;
use std::f32::INFINITY;
use std::io::Write;

pub struct Scene {
    // image
    pub aspect_ratio: f32,
    pub img_width: u32,
    pub img_height: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u8,

    // world
    pub world: HittableList,

    // camera
    pub cam: Camera,
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.,
            img_width: 100,
            img_height: 100,
            samples_per_pixel: 10,
            max_depth: 50,

            world: HittableList::default(),

            cam: Camera::default(),
        }
    }
}

impl Scene {
    pub fn render(&self, buffer: &mut [u8], pitch: usize) {
        let img_height = self.img_height;
        let img_width = self.img_width;
        let world = &self.world;
        let max_depth = self.max_depth;
        let samples_per_pixel = self.samples_per_pixel;
        let cam = &self.cam;

        let mut rng = rand::thread_rng();
        for j in 0..img_height {
            eprint!("\rscanlines remaining: {}", img_width - j);
            std::io::stderr().flush().ok();
            for i in 0..img_width {
                let mut pixel_color = Color::new(0., 0., 0.);
                for _ in 0..samples_per_pixel {
                    let s =
                        (i as f32 + rng.gen::<f32>()) / (img_width - 1) as f32;
                    let t =
                        (j as f32 + rng.gen::<f32>()) / (img_height - 1) as f32;
                    let r = cam.get_ray(s, t);
                    pixel_color += ray_color(&r, world, max_depth);
                }
                writecolor(buffer, pitch, i, j, pixel_color, samples_per_pixel);
            }
        }
        eprintln!("\ndone.");
    }
}

fn ray_color(r: &Ray, world: &HittableList, depth: u8) -> Color {
    if depth == 0 {
        return Color::new(0., 0., 0.);
    }

    if let Some(rec) = world.hit(r, Interval::new(0.001, INFINITY)) {
        if let Some((attentuation, scattered)) = rec.mat.scatter(r, &rec) {
            return attentuation.component_mul(&ray_color(
                &scattered,
                world,
                depth - 1,
            ));
        } else {
            return Color::default();
        }
    }

    let unit_dir = r.dir.normalize();
    let a = 0.5 * (unit_dir.y + 1.);
    (1. - a) * Color::new(1., 1., 1.) + a * Color::new(0.5, 0.7, 1.)
}
