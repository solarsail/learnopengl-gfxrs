use std::collections::HashMap;
use glutin::VirtualKeyCode;
use cgmath::{Matrix4, Point3};
use camera::{Camera, MovementDirection as MD};
use context::Context;

lazy_static! {
    static ref KEY_MAP: HashMap<VirtualKeyCode, MD> = {
        let mut m = HashMap::new();
        m.insert(VirtualKeyCode::W, MD::Up);
        m.insert(VirtualKeyCode::S, MD::Down);
        m.insert(VirtualKeyCode::A, MD::Left);
        m.insert(VirtualKeyCode::D, MD::Right);
        m
    };
}

pub struct CameraSystem {
    camera: Camera,
    yaw: f32,
    pitch: f32,
    sensitivity: f32,
}

impl CameraSystem {
    pub fn new(camera: Camera, sensitivity: f32) -> CameraSystem {
        CameraSystem {
            camera,
            yaw: -90.0,
            pitch: 0.0,
            sensitivity,
        }
    }

    pub fn run(&mut self, ctx: &mut Context, dt: f32) {
        self.camera.update_aspect(
            ctx.screen_width as f32,
            ctx.screen_height as f32,
        );
        for (key, value) in KEY_MAP.iter() {
            self.camera.move_towards(
                *value,
                ctx.key_state.is_pressed(*key),
            );
        }
        let delta = ctx.mouse_state.delta();
        self.yaw += delta.x * self.sensitivity;
        self.pitch -= delta.y * self.sensitivity;
        self.camera.look_around(self.pitch, self.yaw);
        self.camera.zoom(ctx.mouse_state.drain_scroll());
        self.camera.move_for(dt);
        ctx.reset_mouse_pos();
    }

    pub fn projection_matrix(&self) -> Matrix4<f32> {
        self.camera.projection_matrix()
    }

    pub fn view_matrix(&self) -> Matrix4<f32> {
        self.camera.view_matrix()
    }

    #[allow(dead_code)]
    pub fn position(&self) -> Point3<f32> {
        self.camera.pos()
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }
}
