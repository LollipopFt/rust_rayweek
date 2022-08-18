use sdl2::{
    event::Event, keyboard::Scancode, pixels::PixelFormatEnum::RGB24,
    rect::Rect,
};

mod tomain;
use tomain::render;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("raytrace", 1280, 720)
        .position_centered()
        .resizable()
        .vulkan()
        .build()
        .map_err(|x| x.to_string())?;
    let mut canvas = window.into_canvas().build().map_err(|x| x.to_string())?;
    let texture_creator = canvas.texture_creator();

    let width = 256;
    let height = 256;
    let mut texture = texture_creator
        .create_texture_streaming(RGB24, width, height)
        .map_err(|x| x.to_string())?;

    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        render(buffer, pitch, width, height);
    })?;


    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.wait_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown { scancode: Some(Scancode::Escape), .. } => {
                    break 'running
                }
                _ => {}
            }
        canvas.copy(&texture, None, Some(Rect::new(100, 100, width, height)))?;
        canvas.present();
       }
    }

    Ok(())
}
