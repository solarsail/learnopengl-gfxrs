#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;

use std::time;
use gfx::Device;
use glutin::GlContext;
use gfx::traits::FactoryExt;

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

    let (vertex_buffer, slice) =
        factory.create_vertex_buffer_with_slice(model::vertices().as_slice(), ());
    let modifier_buffer = factory.create_constant_buffer(1);

    let mut data = render::pipe::Data {
        vbuf: vertex_buffer,
        modifier: modifier_buffer,
        out: render_target,
    };

    let start_time = time::Instant::now();
    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
            use glutin::WindowEvent::*;
            use glutin::VirtualKeyCode;
            match event {
                glutin::Event::WindowEvent { event, .. } => {
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
                            gfx_window_glutin::update_views(
                                &window,
                                &mut data.out,
                                &mut depth_stencil,
                            );
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        });

        let elapsed = time::Instant::now().duration_since(start_time);
        let tvalue = elapsed.as_secs() as f32 + elapsed.subsec_nanos() as f32 / 1e9;
        let color_mod = [tvalue.sin(), tvalue.cos(), -tvalue.sin()];
        let modifier = render::Modifier { color_mod };
        encoder.clear(&data.out, render::BLACK);
        encoder
            .update_buffer(&data.modifier, &[modifier], 0)
            .unwrap();
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
