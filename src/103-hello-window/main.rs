extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;

use gfx::Device;
use glutin::GlContext;

pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let context = glutin::ContextBuilder::new();
    let builder = glutin::WindowBuilder::new()
        .with_title("Learn OpenGL".to_string())
        .with_dimensions(1024, 768);

    // gfx-rs init
    let (window, mut device, mut factory, mut render_target, mut depth_stencil) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder, context, &events_loop);
    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

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
                                                            &mut render_target,
                                                            &mut depth_stencil);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        });

        encoder.clear(&render_target, BLACK);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
