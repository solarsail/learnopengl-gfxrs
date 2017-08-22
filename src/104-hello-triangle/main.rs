#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;

use gfx::Device;
use glutin::GlContext;
use gfx::traits::FactoryExt;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];


gfx_defines! {
    vertex Vertex {
        pos: [f32; 3] = "aPos",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::RenderTarget<ColorFormat> = "FragColor",
    }
}

impl Vertex {
    fn new(x: f32, y: f32, z: f32) -> Vertex {
        Vertex { pos: [x, y, z] }
    }
}

fn main() {
    let vertices: &[Vertex] = &[Vertex::new(0.5, 0.5, 0.0),
                                Vertex::new(0.5, -0.5, 0.0),
                                Vertex::new(-0.5, -0.5, 0.0),
                                Vertex::new(-0.5, 0.5, 0.0)];

    let indices: &[u16] = &[0, 1, 3, 1, 2, 3];

    let mut events_loop = glutin::EventsLoop::new();
    let context = glutin::ContextBuilder::new();
    let builder = glutin::WindowBuilder::new()
        .with_title("Learn OpenGL".to_string())
        .with_dimensions(1024, 768);

    // gfx-rs init
    let (window, mut device, mut factory, render_target, mut depth_stencil) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder, context, &events_loop);
    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let pso = factory
        .create_pipeline_simple(
            include_bytes!("shader/104.vert"),
            include_bytes!("shader/104.frag"),
            pipe::new(), // instantiates the pipe defined in `gfx_defines!`
        )
        .unwrap();

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(vertices, indices);

    let mut data = pipe::Data {
        vbuf: vertex_buffer,
        out: render_target,
    };

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
                            gfx_window_glutin::update_views(&window,
                                                            &mut data.out,
                                                            &mut depth_stencil);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        });

        encoder.clear(&data.out, BLACK);
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
