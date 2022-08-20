use nalgebra::Vector3;
use std::io::Write;
pub type Color = Vector3<f32>;
pub type Vector = Vector3<f32>;
pub type Point = Vector3<f32>;

mod color;
use color::writecolor;
mod ray;
use ray::Ray;

use crate::Constants;

pub fn render(buffer: &mut [u8], pitch: usize, constants: &Constants) {
    let tonemap: f32 = 255.999;
    let img_height = constants.img_height;
    let img_width = constants.img_width;
    for j in 0..img_height {
        eprint!("\rscanlines remaining: {}", img_width - j);
        std::io::stderr().flush().ok();
        for i in 0..img_width {
            let s = i as f32 / (img_width as f32 - 1.);
            let t = j as f32 / (img_height as f32 - 1.);
            let r = Ray::new(
                constants.origin,
                constants.lower_left
                    + s * constants.horizontal
                    + (1. - t) * constants.vertical
                    - constants.origin,
            );
            let pixel_color = ray_color(&r);
            writecolor(buffer, pitch, i, j, pixel_color, tonemap);
        }
    }
    eprintln!("\ndone.");
}

fn ray_color(r: &Ray) -> Color {
    let unit_dir = r.dir.normalize();
    let a = 0.5 * unit_dir.y + 1.;
    (1. - a) * Color::new(1., 1., 1.) + a * Color::new(0.5, 0.7, 1.)
}
