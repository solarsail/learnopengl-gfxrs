#[macro_use]
extern crate approx;
extern crate cgmath;
extern crate find_folder;
#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate image;

use std::time;
use gfx::Device;
use glutin::GlContext;
use gfx::traits::FactoryExt;
use cgmath::{Deg, Matrix4, Point3, Rad, Vector3};
use cgmath::prelude::*;

mod render;
mod model;
mod camera;

use camera::{CameraBuilder, MovementDirection as MD};


fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let context = glutin::ContextBuilder::new();
    let builder = glutin::WindowBuilder::new()
        .with_title("Learn OpenGL".to_string())
        .with_dimensions(1024, 768);

    // gfx-rs init
    let (window, mut device, mut factory, render_target, depth_stencil) =
        gfx_window_glutin::init::<render::ColorFormat, render::DepthFormat>(
            builder,
            context,
            &events_loop,
        );
    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let pso = factory
        .create_pipeline_simple(
            include_bytes!("shader/vertex.glsl"),
            include_bytes!("shader/fragment.glsl"),
            render::pipe::new(), // instantiates the pipe defined in `gfx_defines!`
        )
        .unwrap();

    let (vertex_buffer, slice) =
        factory.create_vertex_buffer_with_slice(model::vertices().as_slice(), ());
    let translations_buffer = factory.create_constant_buffer(1);
    let texture1 = render::load_texture(&mut factory, "textures/container.jpg");
    let texture2 = render::load_texture(&mut factory, "textures/awesomeface.png");
    let sampler1 = factory.create_sampler_linear();
    let sampler2 = factory.create_sampler_linear();

    let mut data = render::pipe::Data {
        vbuf: vertex_buffer,
        transform: translations_buffer,
        texture1: (texture1, sampler1),
        texture2: (texture2, sampler2),
        out: render_target,
        out_depth: depth_stencil,
    };

    // Game loop
    let _start_time = time::Instant::now();
    let mut last_frame = time::Instant::now();
    let mut running = true;
    let mut width = 1024.0;
    let mut height = 768.0;
    let mut pitch = 0.0;
    let mut yaw = -90.0;
    let sensitivity = 0.1;
    let mut first_mouse = true;
    let mut camera = CameraBuilder::new(Point3::new(0.0, 0.0, 3.0), Vector3::unit_y())
        .aspect(width, height)
        .build();

    while running {
        let current_frame = time::Instant::now();
        let dt = current_frame.duration_since(last_frame);
        last_frame = current_frame;
        events_loop.poll_events(|event| {
            use glutin::WindowEvent::*;
            use glutin::{MouseScrollDelta, VirtualKeyCode};
            use glutin::ElementState::*;
            if let glutin::Event::WindowEvent { event, .. } = event {
                match event {
                    Closed => {
                        running = false; // cannot `break` in closure
                    }
                    KeyboardInput {
                        input: glutin::KeyboardInput {
                            state,
                            virtual_keycode: Some(vk),
                            ..
                        },
                        ..
                    } => match (state, vk) {
                        (Pressed, VirtualKeyCode::W) => camera.move_towards(MD::Up, true),
                        (Pressed, VirtualKeyCode::S) => camera.move_towards(MD::Down, true),
                        (Pressed, VirtualKeyCode::A) => camera.move_towards(MD::Left, true),
                        (Pressed, VirtualKeyCode::D) => camera.move_towards(MD::Right, true),
                        (Released, VirtualKeyCode::W) => camera.move_towards(MD::Up, false),
                        (Released, VirtualKeyCode::S) => camera.move_towards(MD::Down, false),
                        (Released, VirtualKeyCode::A) => camera.move_towards(MD::Left, false),
                        (Released, VirtualKeyCode::D) => camera.move_towards(MD::Right, false),
                        (_, VirtualKeyCode::Escape) => running = false,
                        _ => {}
                    },
                    MouseMoved {
                        position: (mut x, mut y),
                        ..
                    } => {
                        if first_mouse {
                            if relative_eq!(x, width as f64 / 2.0) &&
                                relative_eq!(y, height as f64 / 2.0)
                            {
                                first_mouse = false;
                            } else {
                                x = width as f64 / 2.0;
                                y = height as f64 / 2.0;
                            }
                        }
                        let dx = x as f32 - width / 2.0;
                        let dy = height / 2.0 - y as f32;
                        yaw += dx * sensitivity;
                        pitch += dy * sensitivity;
                        camera.look_around(pitch, yaw);
                        window
                            .set_cursor_position(width as i32 / 2, height as i32 / 2)
                            .unwrap();
                    }
                    Focused(true) => {
                        first_mouse = true;
                    }
                    MouseEntered { .. } => {
                        window
                            .set_cursor_position(width as i32 / 2, height as i32 / 2)
                            .unwrap();
                    }
                    MouseWheel {
                        delta: MouseScrollDelta::LineDelta(_, dy),
                        ..
                    } => {
                        camera.zoom(dy);
                    }
                    Resized(w, h) => {
                        width = w as f32;
                        height = h as f32;
                        camera.update_aspect(width, height);
                        gfx_window_glutin::update_views(
                            &window,
                            &mut data.out,
                            &mut data.out_depth,
                        );
                    }
                    _ => {}
                }
            }
        });

        let tvalue = dt.as_secs() as f32 + dt.subsec_nanos() as f32 / 1e9;
        camera.move_for(tvalue);

        let projection = camera.projection_matrix();
        let view = camera.view_matrix();

        encoder.clear(&data.out, render::BLACK);
        encoder.clear_depth(&data.out_depth, 1.0);
        for (i, pos) in model::cube_pos().into_iter().enumerate() {
            let trans = Matrix4::from_translation(pos);
            let rot = Matrix4::from_axis_angle(
                Vector3::new(0.5, 1.0, 0.0).normalize(),
                Rad::from(Deg(i as f32 * 20.0)),
            );
            let model = trans * rot;
            let translation = render::Transform {
                model: model.into(),
                view: view.into(),
                projection: projection.into(),
            };
            encoder
                .update_buffer(&data.transform, &[translation], 0)
                .unwrap();
            encoder.draw(&slice, &pso, &data);
        }
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
