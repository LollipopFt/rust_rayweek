use std::rc::Rc;

use sdl2::{
    event::{Event, WindowEvent},
    keyboard::Scancode,
    pixels::PixelFormatEnum::RGB24,
    rect::Rect,
};

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod scene;
mod sphere;
mod vec3;

use color::writecolor;
use interval::Interval;
use material::{Dielectric, Lambertian, Metal};
use ray::Ray;
use scene::Scene;
use sphere::Sphere;
use vec3::{Color, Point, Vector};

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

    // image
    let mut scene_desc = Scene {
        aspect_ratio: 16. / 9.,
        img_width: 400,
        samples_per_pixel: 100,
        ..Default::default()
    };
    scene_desc.img_height =
        (scene_desc.img_width as f32 / scene_desc.aspect_ratio) as u32;

    // world
    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.));

    scene_desc.world.push(Box::new(Sphere::new(
        Point::new(0., -100.5, -1.),
        100.,
        material_ground,
    )));
    scene_desc.world.push(Box::new(Sphere::new(
        Point::new(0., 0., -1.),
        0.5,
        material_center,
    )));
    scene_desc.world.push(Box::new(Sphere::new(
        Point::new(-1., 0., -1.),
        0.5,
        material_left.clone(),
    )));
    scene_desc.world.push(Box::new(Sphere::new(
        Point::new(-1., 0., -1.),
        -0.4,
        material_left,
    )));
    scene_desc.world.push(Box::new(Sphere::new(
        Point::new(1., 0., -1.),
        0.5,
        material_right,
    )));

    let mut texture = texture_creator
        .create_texture_streaming(
            RGB24,
            scene_desc.img_width,
            scene_desc.img_height,
        )
        .map_err(|x| x.to_string())?;

    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        scene_desc.cam.init(scene_desc.aspect_ratio);
        scene_desc.render(buffer, pitch);
    })?;

    canvas.copy(
        &texture,
        None,
        Some(Rect::new(
            0,
            0,
            2 * scene_desc.img_width,
            2 * scene_desc.img_height,
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
                            2 * scene_desc.img_width,
                            2 * scene_desc.img_height,
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
