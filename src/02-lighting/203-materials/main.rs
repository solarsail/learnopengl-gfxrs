#[macro_use]
extern crate approx;
extern crate cgmath;
#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
#[macro_use]
extern crate lazy_static;

use std::time;
use gfx::Device;
use glutin::GlContext;
use gfx::traits::FactoryExt;
use cgmath::{Matrix4, Point3, Vector3};
use cgmath::prelude::*;

mod render;
mod model;
mod camera;
mod context;
mod system;

use system::CameraSystem;
use context::Context;
use camera::CameraBuilder;


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

    let obj_pso = factory
        .create_pipeline_simple(
            include_bytes!("shader/vertex.glsl"),
            include_bytes!("shader/obj_fragment.glsl"),
            render::obj_pipe::new(),
        )
        .unwrap();
    let light_pso = factory
        .create_pipeline_simple(
            include_bytes!("shader/vertex.glsl"),
            include_bytes!("shader/light_fragment.glsl"),
            render::light_pipe::new(),
        )
        .unwrap();

    let (obj_vertex_buffer, obj_slice) =
        factory.create_vertex_buffer_with_slice(model::vertices().as_slice(), ());
    let (light_vertex_buffer, light_slice) =
        factory.create_vertex_buffer_with_slice(model::vertices().as_slice(), ());
    let obj_trans_buffer = factory.create_constant_buffer(1);
    let light_trans_buffer = factory.create_constant_buffer(1);
    let obj_light_buffer = factory.create_constant_buffer(1);
    let obj_material_buffer = factory.create_constant_buffer(1);

    let mut obj_data = render::obj_pipe::Data {
        vbuf: obj_vertex_buffer,
        transform: obj_trans_buffer,
        light: obj_light_buffer,
        material: obj_material_buffer,
        out: render_target.clone(),
        out_depth: depth_stencil.clone(),
    };
    let mut light_data = render::light_pipe::Data {
        vbuf: light_vertex_buffer,
        transform: light_trans_buffer,
        out: render_target.clone(),
        out_depth: depth_stencil.clone(),
    };

    // Game loop
    let mut last_frame = time::Instant::now();
    let mut dt;
    let mut running = true;
    let camera = CameraBuilder::new(Point3::new(0.0, 0.0, 3.0), Vector3::unit_y())
        .aspect(1024.0, 768.0)
        .build();

    let light_pos = Vector3::new(1.2, 1.0, 2.0);

    let mut context = Context::new(1024.0, 768.0);
    let mut cs = CameraSystem::new(camera, 0.1);

    while running {
        let current_frame = time::Instant::now();
        dt = current_frame.duration_since(last_frame);
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
                        (_, VirtualKeyCode::Escape) => running = false,
                        _ => {
                            context.key_state.update_key(vk, state == Pressed);
                        }
                    },
                    MouseMoved {
                        position: (x, y), ..
                    } => {
                        context.update_mouse_pos(&window, x as f32, y as f32);
                    }
                    Focused(true) => {
                        context.focused();
                    }
                    MouseEntered { .. } => {
                        context.mouse_entered(&window);
                    }
                    MouseWheel {
                        delta: MouseScrollDelta::LineDelta(_, dy),
                        ..
                    } => {
                        context.mouse_state.update_scroll(dy);
                    }
                    Resized(w, h) => {
                        context.update_dimensions(w as f32, h as f32);
                        gfx_window_glutin::update_views(
                            &window,
                            &mut obj_data.out,
                            &mut obj_data.out_depth,
                        );
                        gfx_window_glutin::update_views(
                            &window,
                            &mut light_data.out,
                            &mut light_data.out_depth,
                        );
                    }
                    _ => {}
                }
            }
        });

        let tvalue = dt.as_secs() as f32 + dt.subsec_nanos() as f32 / 1e9;
        cs.run(&mut context, tvalue);

        let projection = cs.projection_matrix();
        let view = cs.view_matrix();
        let obj_model = Matrix4::identity();
        let trans = Matrix4::from_translation(light_pos);
        let scale = Matrix4::from_scale(0.2);
        let light_model = trans * scale;
        let obj_translation = render::Transform {
            model: obj_model.into(),
            view: view.into(),
            projection: projection.into(),
        };
        let light_translation = render::Transform {
            model: light_model.into(),
            view: view.into(),
            projection: projection.into(),
        };
        let obj_light = render::Lighting::new(
            [1.0, 0.5, 0.31],
            [1.0, 1.0, 1.0],
            light_pos.into(),
            cs.position().into(),
        );
        let obj_material = render::Material::new(
            [1.0, 0.5, 0.31],
            [1.0, 0.5, 0.31],
            [0.5, 0.5, 0.5],
            32.0
        );

        encoder.clear(&render_target, render::BLACK);
        encoder.clear_depth(&depth_stencil, 1.0);
        encoder
            .update_buffer(&obj_data.transform, &[obj_translation], 0)
            .unwrap();
        encoder
            .update_buffer(&obj_data.light, &[obj_light], 0)
            .unwrap();
        encoder
            .update_buffer(&obj_data.material, &[obj_material], 0)
            .unwrap();
        encoder
            .update_buffer(&light_data.transform, &[light_translation], 0)
            .unwrap();
        encoder.draw(&obj_slice, &obj_pso, &obj_data);
        encoder.draw(&light_slice, &light_pso, &light_data);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
