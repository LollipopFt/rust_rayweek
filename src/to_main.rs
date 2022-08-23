use nalgebra::Vector3;
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

use crate::Constants;

pub fn render(buffer: &mut [u8], pitch: usize, constants: &Constants) {
    let tonemap: f32 = 255.999;
    let c = constants;
    for j in 0..c.img_height {
        eprint!("\rscanlines remaining: {}", c.img_width - j);
        std::io::stderr().flush().ok();
        for i in 0..c.img_width {
            let s = i as f32 / (c.img_width as f32 - 1.);
            let t = j as f32 / (c.img_height as f32 - 1.);
            let r = Ray::new(
                c.origin,
                c.lower_left + s * c.horizontal + (1. - t) * c.vertical
                    - c.origin,
            );
            let pixel_color = ray_color(&r, &c.world);
            writecolor(buffer, pitch, i, j, pixel_color, tonemap);
        }
    }
    eprintln!("\ndone.");
}

fn ray_color(r: &Ray, world: &HittableList) -> Color {
    if let Some(rec) = world.hit(r, 0., INFINITY) {
        return 0.5 * (rec.normal + Color::new(1., 1., 1.));
    }
    let unit_dir = r.dir.normalize();
    let a = 0.5 * (unit_dir.y + 1.);
    (1. - a) * Color::new(1., 1., 1.) + a * Color::new(0.5, 0.7, 1.)
}
