#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate image;
extern crate find_folder;

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

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(
        model::vertices().as_slice(),
        model::indices().as_slice(),
    );
    let texture1 = render::load_texture(&mut factory, "textures/container.jpg");
    let texture2 = render::load_texture(&mut factory, "textures/awesomeface.png");
    let sampler1 = factory.create_sampler_linear();
    let sampler2 = factory.create_sampler_linear();

    let mut data = render::pipe::Data {
        vbuf: vertex_buffer,
        texture1: (texture1, sampler1),
        texture2: (texture2, sampler2),
        out: render_target,
    };

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

        encoder.clear(&data.out, render::BLACK);
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
