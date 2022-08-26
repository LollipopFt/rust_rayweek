use sdl2::{
    event::{Event, WindowEvent},
    keyboard::Scancode,
    pixels::PixelFormatEnum::RGB24,
    rect::Rect,
};

mod to_main;
use to_main::{render, HittableList, Point, Sphere, Vector, Camera};

#[derive(Default)]
pub struct Constants {
    // image
    aspect_ratio: f32,
    img_width: u32,
    img_height: u32,
    samples_per_pixel: u32,

    // world
    world: HittableList,

    // camera
    cam: Camera,
    origin: Point,
    horizontal: Vector,
    vertical: Vector,
    lower_left: Point,
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("raytrace", 800, 450)
        .position_centered()
        .vulkan()
        .resizable()
        .maximized()
        .allow_highdpi()
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
    constants.samples_per_pixel = 100;

    // world
    constants.world = HittableList::new();
    constants.world.push(Box::new(Sphere::new(Point::new(0., 0., -1.), 0.5)));
    constants
        .world
        .push(Box::new(Sphere::new(Point::new(0., -100.5, -1.), 100.)));

    // camera
    let viewport_height = 2.;
    let viewport_width =
        constants.aspect_ratio * viewport_height;
    let focal_length = 1.;

    constants.origin = Point::new(0., 0., 0.);
    constants.horizontal = Vector::new(viewport_width, 0., 0.);
    constants.vertical = Vector::new(0., viewport_height, 0.);
    constants.lower_left = constants.origin
        - constants.horizontal / 2.
        - constants.vertical / 2.
        - Vector::new(0., 0., focal_length);

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
    canvas.copy(
        &texture,
        None,
        Some(Rect::new(
            0,
            0,
            2 * constants.img_width,
            2 * constants.img_height,
        )),
    )?;
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.wait_iter() {
            use Scancode::Escape;
            use WindowEvent::{Maximized, Resized};

            match event {
                Event::Quit { .. }
                | Event::KeyDown { scancode: Some(Escape), .. } => {
                    break 'running
                }
                Event::Window { win_event: Resized(..), .. }
                | Event::Window { win_event: Maximized, .. } => {
                    canvas.copy(
                        &texture,
                        None,
                        Some(Rect::new(
                            0,
                            0,
                            2 * constants.img_width,
                            2 * constants.img_height,
                        )),
                    )?;
                    canvas.present();
                }
                _ => {}
            }
        }
    }

    Ok(())
}
