pub fn render(buffer: &mut [u8], pitch: usize, width: u32, height: u32) {
    let tonemap: f32 = 255.999;
    for j in (0..height).rev() {
        for i in 0..width {
            let r = i as f32 / (width as f32 - 1.);
            let g = j as f32 / (height as f32 - 1.);
            let b = 0.25f32;

            let ir = (tonemap * r) as u8;
            let ig = (tonemap * g) as u8;
            let ib = (tonemap * b) as u8;
            setpixel(buffer, pitch, i, height-1-j, [ir, ig, ib]);
        }
    }
}

fn setpixel(buffer: &mut [u8], pitch: usize, width: u32, height: u32, color: [u8; 3]) {
    let offset = height as usize * pitch + width as usize * 3;
    buffer[offset] = color[0];
    buffer[offset + 1] = color[1];
    buffer[offset + 2] = color[2];
}
