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
            let pixel_color = ray_color(&r);
            writecolor(buffer, pitch, i, j, pixel_color, tonemap);
        }
    }
    eprintln!("\ndone.");
}

fn ray_color(r: &Ray) -> Color {
    let t = hit_sphere(&Point::new(0., 0., -1.), 0.5, r);
    if t > 0. {
        let n = (r.at(t) - Vector::new(0., 0., -1.)).normalize();
        return 0.5 * Color::new(n.x + 1., n.y + 1., n.z + 1.);
    }
    let unit_dir = r.dir.normalize();
    let a = 0.5 * (unit_dir.y + 1.);
    (1. - a) * Color::new(1., 1., 1.) + a * Color::new(0.5, 0.7, 1.)
}

fn hit_sphere(center: &Point, radius: f32, r: &Ray) -> f32 {
    // ray equation: P(t) = A + tb
    // in a sphere: (P(t)-C)∙(P(t)-C) = r² => (A+tb-C)∙(a+tb-C) = r²
    // t²b∙b + 2tb∙(A-C) + (A-C)∙(A-C) = r²
    // a: b∙b, b: 2b∙(A-C), c: (A-C)∙(A-C) - r²
    let oc = r.origin - center;
    let a = r.dir.dot(&r.dir);
    let b = 2. * oc.dot(&r.dir);
    let c = oc.dot(&oc) - radius*radius;
    let discriminant = b*b - 4.*a*c;
    if discriminant < 0. {
        -1.
    } else {
        (-b - discriminant.sqrt()) / (2. * a)
    }
}
