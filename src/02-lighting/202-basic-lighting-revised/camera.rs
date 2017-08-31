use cgmath::prelude::*;
use cgmath::{Deg, Matrix4, PerspectiveFov, Point3, Vector3};


#[derive(Debug, Copy, Clone)]
pub enum MovementDirection {
    Up,
    Down,
    Left,
    Right,
}

pub struct Camera {
    pos: Point3<f32>,
    front: Vector3<f32>,
    right: Vector3<f32>,
    up: Vector3<f32>,
    world_up: Vector3<f32>,
    speed: f32,
    yaw: f32,
    pitch: f32,
    aspect: f32,
    fov: f32,
    movement: [f32; 4],
}

impl Camera {
    fn update_vectors(&mut self) {
        self.front = Vector3::new(
            Deg(self.yaw).cos() * Deg(self.pitch).cos(),
            Deg(self.pitch).sin(),
            Deg(self.yaw).sin() * Deg(self.pitch).cos(),
        ).normalize();
        self.right = self.front.cross(self.world_up).normalize();
        self.up = self.right.cross(self.front).normalize();
    }

    fn movement_vector(&self) -> Option<Vector3<f32>> {
        let x = (self.movement[2] - self.movement[3]) * self.right;
        let z = (self.movement[0] - self.movement[1]) * self.front;
        let v = x + z;
        if !v.is_zero() {
            Some(v.normalize())
        } else {
            None
        }
    }

    pub fn move_towards(&mut self, dir: MovementDirection, m: bool) {
        let index = match dir {
            MovementDirection::Up => 0,
            MovementDirection::Down => 1,
            MovementDirection::Left => 3,
            MovementDirection::Right => 2,
        };
        self.movement[index] = if m { 1.0 } else { 0.0 };
    }

    #[allow(dead_code)]
    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed;
    }

    pub fn move_for(&mut self, ts: f32) {
        if let Some(v) = self.movement_vector() {
            self.pos += v * self.speed * ts;
        }
    }

    #[allow(dead_code)]
    pub fn look_around(&mut self, mut pitch: f32, yaw: f32) {
        if pitch > 89.0 {
            pitch = 89.0;
        }
        if pitch < -89.0 {
            pitch = -89.0;
        }
        self.pitch = pitch;
        self.yaw = yaw;
        self.update_vectors();
    }

    pub fn view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at(self.pos, self.pos + self.front, self.up)
    }

    #[allow(dead_code)]
    pub fn update_aspect(&mut self, width: f32, height: f32) {
        self.aspect = width / height;
    }

    pub fn zoom(&mut self, zoom: f32) {
        self.fov -= zoom;
        if self.fov > 45.0 {
            self.fov = 45.0
        }
        if self.fov < 1.0 {
            self.fov = 1.0
        }
    }

    pub fn projection_matrix(&self) -> Matrix4<f32> {
        PerspectiveFov {
            fovy: Deg(self.fov).into(),
            aspect: self.aspect,
            near: 0.1,
            far: 100.0,
        }.into()
    }

    #[allow(dead_code)]
    pub fn pos(&self) -> Point3<f32> {
        self.pos
    }
}

pub struct CameraBuilder {
    camera: Camera,
}

impl CameraBuilder {
    pub fn new(pos: Point3<f32>, up: Vector3<f32>) -> CameraBuilder {
        let camera = Camera {
            pos,
            front: Vector3::zero(),
            up,
            right: Vector3::zero(),
            world_up: up,
            speed: 2.5,
            yaw: -90.0,
            pitch: 0.0,
            aspect: 4.0 / 3.0,
            fov: 45.0,
            movement: [0.0, 0.0, 0.0, 0.0],
        };
        CameraBuilder { camera }
    }

    #[allow(dead_code)]
    pub fn pitch(mut self, pitch: f32) -> CameraBuilder {
        self.camera.pitch = pitch;
        self
    }

    #[allow(dead_code)]
    pub fn yaw(mut self, yaw: f32) -> CameraBuilder {
        self.camera.yaw = yaw;
        self
    }

    pub fn aspect(mut self, width: f32, height: f32) -> CameraBuilder {
        self.camera.aspect = width / height;
        self
    }

    pub fn build(mut self) -> Camera {
        self.camera.update_vectors();
        self.camera
    }
}
