#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate image;
extern crate cgmath;
extern crate find_folder;

use std::time;
use gfx::Device;
use glutin::GlContext;
use gfx::traits::FactoryExt;
use cgmath::{Vector3, Matrix4, Rad};

mod render;
mod model;


fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let context = glutin::ContextBuilder::new();
    let builder = glutin::WindowBuilder::new()
        .with_title("Learn OpenGL".to_string())
        .with_dimensions(1024, 768);

    // gfx-rs init
    let (window, mut device, mut factory, render_target, mut depth_stencil) =
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

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(
        model::vertices().as_slice(),
        model::indices().as_slice(),
    );
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
    };

    let start_time = time::Instant::now();
    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
            use glutin::WindowEvent::*;
            use glutin::VirtualKeyCode;
            if let glutin::Event::WindowEvent { event, .. } = event {
                match event {
                    KeyboardInput {
                        input: glutin::KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape), ..
                        },
                        ..
                    } |
                    Closed => {
                        running = false; // cannot `break` in closure
                    }
                    Resized(_width, _height) => {
                        gfx_window_glutin::update_views(&window, &mut data.out, &mut depth_stencil);
                    }
                    _ => {}
                }
            }
        });

        let elapsed = time::Instant::now().duration_since(start_time);
        let tvalue = elapsed.as_secs() as f32 + elapsed.subsec_nanos() as f32 / 1e9;
        let translation = Matrix4::from_translation(Vector3::new(0.5, -0.5, 0.0));
        let rotation = Matrix4::from_angle_z(Rad(tvalue));
        let trans = translation * rotation;
        let translation = render::Transform { trans: trans.into() };
        encoder.clear(&data.out, render::BLACK);
        encoder
            .update_buffer(&data.transform, &[translation], 0)
            .unwrap();
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
