use nalgebra::Vector3;
use std::io::Write;
pub type Color = Vector3<f32>;
pub type Vector = Vector3<f32>;
pub type Point = Vector3<f32>;

mod color;
use color::writecolor;

pub fn render(
    buffer: &mut [u8],
    pitch: usize,
    img_width: u32,
    img_height: u32,
) {
    let tonemap: f32 = 255.999;
    for j in 0..img_height {
        eprint!("\rscanlines remaining: {}", img_width - j);
        std::io::stderr().flush().ok();
        for i in 0..img_width {
            let pixel_color = Color::new(
                i as f32 / (img_width as f32 - 1.),
                j as f32 / (img_height as f32 - 1.),
                0.25f32,
            );
            writecolor(buffer, pitch, i, j, pixel_color, tonemap);
        }
    }
    eprintln!("\ndone.");
}
