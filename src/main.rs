use std::rc::Rc;

use rand::Rng;
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
use material::{Dielectric, Lambertian, Material, Metal};
use ray::Ray;
use scene::Scene;
use sphere::Sphere;
use vec3::{Color, Extensions, Point, Vector};

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
    let mut scene_desc = Scene::default();

    random_spheres(&mut scene_desc);
    scene_desc.img_height =
        (scene_desc.img_width as f32 / scene_desc.aspect_ratio) as u32;

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

fn random_spheres(scene: &mut Scene) {
    scene.aspect_ratio = 16. / 9.;
    scene.img_width = 1200;
    scene.samples_per_pixel = 50;

    scene.cam.lookfrom = Point::new(13., 2., 3.);
    scene.cam.lookat = Point::new(0., 0., 0.);
    scene.cam.vup = Vector::new(0., 1., 0.);
    scene.cam.vfov = 50.;
    scene.cam.aperture = 0.1;
    scene.cam.focus_dist = 10.;

    // world
    let world = &mut scene.world;

    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.push(
        Sphere::new(Point::new(0., -1000., 0.), 1000., ground_material).boxd(),
    );

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f32 = rng.gen();
            let center = Point::new(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );

            if (center - Point::new(4., 0.2, 0.)).norm() > 9. {
                let sphere_material: Rc<dyn Material>;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo =
                        Color::random().component_mul(&Color::random());
                    sphere_material = Rc::new(Lambertian::new(albedo));
                    world
                        .push(Sphere::new(center, 0.2, sphere_material).boxd());
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::rand(0.5, 1.);
                    let fuzz = rng.gen_range(0.0..0.5);
                    sphere_material = Rc::new(Metal::new(albedo, fuzz));
                    world
                        .push(Sphere::new(center, 0.2, sphere_material).boxd());
                } else {
                    // glass
                    sphere_material = Rc::new(Dielectric::new(1.5));
                    world
                        .push(Sphere::new(center, 0.2, sphere_material).boxd());
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.push(Sphere::new(Point::new(0., 1., 0.), 1., material1).boxd());

    let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.push(Sphere::new(Point::new(-4., 1., 0.), 1., material2).boxd());

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.));
    world.push(Sphere::new(Point::new(4., 1., 0.), 1., material3).boxd());
}
