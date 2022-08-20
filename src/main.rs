use sdl2::{
    event::Event, keyboard::Scancode, pixels::PixelFormatEnum::RGB24,
    rect::Rect,
};

mod to_main;
use to_main::{render, Point, Vector};

#[derive(Default)]
pub struct Constants {
    // image
    aspect_ratio: f32,
    img_width: u32,
    img_height: u32,

    // camera
    viewport_height: f32,
    viewport_width: f32,
    focal_length: f32,

    origin: Point,
    horizontal: Vector,
    vertical: Vector,
    lower_left: Point,
}

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

    let mut constants = Constants::default();
    // image
    constants.aspect_ratio = 16. / 9.;
    constants.img_width = 400;
    constants.img_height =
        (constants.img_width as f32 / constants.aspect_ratio) as u32;

    // camera
    constants.viewport_height = 2.;
    constants.viewport_width =
        constants.aspect_ratio * constants.viewport_height;
    constants.focal_length = 1.;

    constants.origin = Point::new(0., 0., 0.);
    constants.horizontal = Vector::new(constants.viewport_width, 0., 0.);
    constants.vertical = Vector::new(0., constants.viewport_height, 0.);
    constants.lower_left = constants.origin
        - constants.horizontal / 2.
        - constants.vertical / 2.
        - Vector::new(0., 0., constants.focal_length);

    let mut texture = texture_creator
        .create_texture_streaming(
            RGB24,
            constants.img_width,
            constants.img_height,
        )
        .map_err(|x| x.to_string())?;

    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        render(buffer, pitch, &constants);
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
            canvas.copy(
                &texture,
                None,
                Some(Rect::new(
                    0,
                    0,
                    constants.img_width,
                    constants.img_height,
                )),
            )?;
            canvas.present();
        }
    }

    Ok(())
}
