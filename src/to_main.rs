use nalgebra::Vector3;
use rand::Rng;
use std::{f32::INFINITY, io::Write};
pub type Color = Vector3<f32>;
pub type Vector = Vector3<f32>;
pub type Point = Vector3<f32>;

mod color;
use color::writecolor;
mod ray;
use ray::Ray;
mod hittable;
pub use hittable::Hit;
mod hittable_list;
pub use hittable_list::HittableList;
mod sphere;
pub use sphere::Sphere;
mod interval;
use interval::Interval;
mod camera;
pub use camera::Camera;
mod material;
pub use material::{Dielectric, Lambertian, Metal};
mod vec3;

use super::Constants;

pub fn render(buffer: &mut [u8], pitch: usize, constants: &Constants) {
    let c = constants;
    let mut rng = rand::thread_rng();
    for j in 0..c.img_height {
        eprint!("\rscanlines remaining: {}", c.img_width - j);
        std::io::stderr().flush().ok();
        for i in 0..c.img_width {
            let mut pixel_color = Color::new(0., 0., 0.);
            for _ in 0..c.samples_per_pixel {
                let s =
                    (i as f32 + rng.gen::<f32>()) / (c.img_width - 1) as f32;
                let t =
                    (j as f32 + rng.gen::<f32>()) / (c.img_height - 1) as f32;
                let r = c.cam.get_ray(s, t);
                pixel_color += ray_color(&r, &c.world, c.max_depth);
            }
            writecolor(buffer, pitch, i, j, pixel_color, c.samples_per_pixel);
        }
    }
    eprintln!("\ndone.");
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
