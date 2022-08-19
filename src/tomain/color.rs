use super::Color;

pub fn writecolor(buffer: &mut [u8], pitch: usize, x: u32, y: u32, color: Color, tonemap: f32) {
    let offset = y as usize * pitch + x as usize * 3;
    buffer[offset] = (tonemap * color[0]) as u8;
    buffer[offset + 1] = (tonemap * color[1]) as u8;
    buffer[offset + 2] = (tonemap * color[2]) as u8;
}
