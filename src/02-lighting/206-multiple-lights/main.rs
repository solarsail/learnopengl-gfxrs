extern crate cgmath;
extern crate find_folder;
#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate gfx_device_gl;
extern crate glutin;
extern crate image;
#[macro_use]
extern crate lazy_static;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
extern crate spin_sleep;

//use std::time;
use gfx::Device;
use glutin::GlContext;
use cgmath::{Matrix4, Point3, Vector3, Rad};
use cgmath::prelude::*;
use structopt::StructOpt;

mod render;
mod model;
mod camera;
mod context;
mod system;

use system::CameraSystem;
use context::Context;
use camera::CameraBuilder;


const SCREEN_WIDTH: i32 = 1024;
const SCREEN_HEIGHT: i32 = 768;

#[derive(StructOpt)]
#[structopt(name = "light-casters", about = "Multiple light casters")]
struct Opt {
    #[structopt(long = "directional", help = "Enable directional lights")]
    dir: Option<bool>,
    #[structopt(long = "point", help = "Enable point lights")]
    point: Option<bool>,
    #[structopt(long = "spot", help = "Enable spot light")]
    spot: Option<bool>,
}

fn main() {
    let opt = Opt::from_args();

    let mut events_loop = glutin::EventsLoop::new();
    let context = glutin::ContextBuilder::new();
    let builder = glutin::WindowBuilder::new()
        .with_title("Learn OpenGL".to_string())
        .with_dimensions(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32);

    // gfx-rs init
    let (window, mut device, mut factory, render_target, depth_stencil) =
        gfx_window_glutin::init::<render::ColorFormat, render::DepthFormat>(
            builder,
            context,
            &events_loop,
        );
    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let material = render::Material::new(
        &mut factory,
        "textures/container2.png",
        "textures/container2_specular.png",
        32.0,
    );
    let rot_axis = Vector3::new(1.0, 0.3, 0.5).normalize();
    let cubes: Vec<_> = model::cube_positions()
        .into_iter()
        .enumerate()
        .map(|(i, pos)| {
            render::Object::new(
                &mut factory,
                model::vertices(),
                Matrix4::from_translation(pos) *
                    Matrix4::from_axis_angle(rot_axis, Rad(20.0 * i as f32)),
                material.clone(),
            )
        })
        .collect();

    let cube_brush = render::ObjectBrush::new(&mut factory);
    let lamp_brush = render::LampBrush::new(&mut factory);

    let light_color = Vector3::new(1.0, 1.0, 1.0);
    let scale = Matrix4::from_scale(0.2);

    let dir_lights: Vec<_> = model::light_directions()
        .into_iter()
        .map(|dir| {
            render::DirLight::new((light_color * 0.05), (light_color * 0.3), light_color, dir)
        })
        .collect();

    let point_lights: Vec<_> = model::light_positions()
        .into_iter()
        .map(|pos| {
            render::PointLight::new(light_color * 0.05, light_color * 0.3, light_color, pos)
        })
        .collect();

    let light_args = render::LightArgs {
        num_dir: if let Some(false) = opt.dir {
            0
        } else {
            model::light_directions().len() as i32
        },
        num_point: if let Some(false) = opt.point {
            0
        } else {
            model::light_positions().len() as i32
        },
    };

    let lamps: Vec<_> = model::light_positions()
        .into_iter()
        .map(|pos| {
            render::Lamp::new(
                &mut factory,
                model::vertices(),
                Matrix4::from_translation(pos) * scale,
                light_color,
            )
        })
        .collect();

    // Game loop
    //let start_time = time::Instant::now();
    let mut loop_helper = spin_sleep::LoopHelper::builder()
       .report_interval_s(0.5) // report every half a second
       .build_with_target_rate(60.0); // limit to 60 FPS if possible

    let mut current_fps = None;

    let camera = CameraBuilder::new(Point3::new(0.0, 0.0, 3.0), Vector3::unit_y())
        .aspect(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32)
        .build();


    let mut context = Context::new(
        &window,
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        render_target,
        depth_stencil,
    );
    let mut cs = CameraSystem::new(camera, 0.1);

    while context.running {
        let delta = loop_helper.loop_start(); // or .loop_start_s() for f64 seconds
        //let elapsed = current_frame.duration_since(start_time);
        events_loop.poll_events(|event| { update(event, &mut context); });

        let dt = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1e9;
        //let elapsed = elapsed.as_secs() as f32 + elapsed.subsec_nanos() as f32 / 1e9;
        cs.run(&mut context, dt);

        if let Some(fps) = loop_helper.report_rate() {
            current_fps = Some(fps);
        }

        let camera = cs.camera();
        encoder.clear(&context.render_target, render::BG);
        encoder.clear_depth(&context.depth_stencil, 1.0);
        for cube in cubes.iter() {
            cube_brush.draw(
                &cube,
                &dir_lights,
                &point_lights,
                &light_args,
                camera,
                &context.render_target,
                &context.depth_stencil,
                &mut encoder,
            );
        }
        for lamp in lamps.iter() {
            lamp_brush.draw(
                &lamp,
                camera,
                &context.render_target,
                &context.depth_stencil,
                &mut encoder,
            );
        }
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
        loop_helper.loop_sleep(); // sleeps to acheive a 60 FPS rate
    }
}

fn update(event: glutin::Event, ctx: &mut Context) {
    use glutin::WindowEvent::*;
    use glutin::{MouseScrollDelta, VirtualKeyCode};
    use glutin::ElementState::*;
    if let glutin::Event::WindowEvent { event, .. } = event {
        match event {
            Closed => {
                ctx.running = false; // cannot `break` in closure
            }
            KeyboardInput {
                input: glutin::KeyboardInput {
                    state,
                    virtual_keycode: Some(vk),
                    ..
                },
                ..
            } => {
                match (state, vk) {
                    (_, VirtualKeyCode::Escape) => ctx.running = false,
                    _ => {
                        ctx.key_state.update_key(vk, state == Pressed);
                    }
                }
            }
            MouseMoved { position: (x, y), .. } => {
                ctx.update_mouse_pos(x as i32, y as i32);
            }
            Focused(true) => {
                ctx.focused();
            }
            MouseEntered { .. } => {
                ctx.mouse_entered();
            }
            MouseWheel { delta: MouseScrollDelta::LineDelta(_, dy), .. } => {
                ctx.mouse_state.update_scroll(dy);
            }
            Resized(w, h) => {
                ctx.update_dimensions(w, h);
                gfx_window_glutin::update_views(
                    &ctx.window,
                    &mut ctx.render_target,
                    &mut ctx.depth_stencil,
                );
            }
            _ => {}
        }
    }
}
