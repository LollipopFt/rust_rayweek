use std::io::Write;

pub fn render(
    buffer: &mut [u8],
    pitch: usize,
    img_width: u32,
    img_height: u32,
) {
    let tonemap: f32 = 255.999;
    for j in 0..img_height {
        eprint!("\rscanlines remaining: {j}");
        std::io::stderr().flush().ok();
        for i in 0..img_width {
            let r = i as f32 / (img_width as f32 - 1.);
            let g = j as f32 / (img_height as f32 - 1.);
            let b = 0.25f32;

            let ir = (tonemap * r) as u8;
            let ig = (tonemap * g) as u8;
            let ib = (tonemap * b) as u8;
            setpixel(buffer, pitch, i, j, [ir, ig, ib]);
        }
    }
    eprintln!("\ndone.");
}

fn setpixel(buffer: &mut [u8], pitch: usize, x: u32, y: u32, color: [u8; 3]) {
    let offset = y as usize * pitch + x as usize * 3;
    buffer[offset] = color[0];
    buffer[offset + 1] = color[1];
    buffer[offset + 2] = color[2];
}
