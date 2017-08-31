use std::collections::HashMap;
use glutin::{VirtualKeyCode};
use cgmath::{Point3, Matrix4};
use camera::{Camera, MovementDirection as MD};
use context::Context;

pub struct CameraSystem {
    camera: Camera,
    key_map: HashMap<VirtualKeyCode, MD>,
    yaw: f32,
    pitch: f32,
    sensitivity: f32,
}

impl CameraSystem {
    pub fn new(camera: Camera, sensitivity: f32) -> CameraSystem {
        let mut key_map = HashMap::new();
        // TODO: use phf
        key_map.insert(VirtualKeyCode::W, MD::Up);
        key_map.insert(VirtualKeyCode::S, MD::Down);
        key_map.insert(VirtualKeyCode::A, MD::Left);
        key_map.insert(VirtualKeyCode::D, MD::Right);
        CameraSystem {
            camera,
            key_map,
            yaw: -90.0,
            pitch: 0.0,
            sensitivity,
        }
    }

    pub fn run(&mut self, ctx: &mut Context, dt: f32) {
        self.camera.update_aspect(ctx.screen_width, ctx.screen_height);
        for (key, value) in self.key_map.iter() {
            self.camera.move_towards(*value, ctx.key_state.is_pressed(*key));
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

    pub fn position(&self) -> Point3<f32> {
        self.camera.pos()
    }
}