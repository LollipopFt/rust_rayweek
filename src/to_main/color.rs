use super::{Color, interval::Interval};

pub fn writecolor(
    buffer: &mut [u8],
    pitch: usize,
    x: u32,
    y: u32,
    pixel_color: Color,
    samples_per_pixel: u32,
) {
    let r = pixel_color.x;
    let g = pixel_color.y;
    let b = pixel_color.z;

    // divide color by number of samples & gamma correct for gamma = 2
    let scale = 1. / samples_per_pixel as f32;
    let r = (r * scale).sqrt();
    let g = (g * scale).sqrt();
    let b = (b * scale).sqrt();

    let intensity = Interval::new(0., 0.999);
    let offset = y as usize * pitch + x as usize * 3;
    buffer[offset] = (256. * intensity.clamp(r)) as u8;
    buffer[offset + 1] = (256. * intensity.clamp(g)) as u8;
    buffer[offset + 2] = (256. * intensity.clamp(b)) as u8;
}
