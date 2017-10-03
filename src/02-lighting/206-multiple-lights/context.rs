use std::collections::HashSet;
use cgmath::{Point2, Vector2};
use glutin::{GlWindow, VirtualKeyCode};
use gfx::handle::{DepthStencilView, RenderTargetView};
use gfx_device_gl::Resources as R;
use render;


pub struct KeyState {
    pressed: HashSet<VirtualKeyCode>,
}

impl KeyState {
    pub fn new() -> KeyState {
        KeyState { pressed: HashSet::new() }
    }

    pub fn update_key(&mut self, key: VirtualKeyCode, pressed: bool) {
        if pressed {
            self.pressed.insert(key);
        } else {
            self.pressed.remove(&key);
        }
    }

    pub fn is_pressed(&self, key: VirtualKeyCode) -> bool {
        self.pressed.contains(&key)
    }
}

pub struct MouseState {
    center: Point2<f32>,
    delta: Vector2<f32>,
    scroll: f32,
}

impl MouseState {
    pub fn new() -> MouseState {
        MouseState {
            center: Point2::new(0.0, 0.0),
            delta: Vector2::new(0.0, 0.0),
            scroll: 0.0,
        }
    }

    pub fn update_position(&mut self, x: i32, y: i32) {
        self.delta = Point2::new(x as f32, y as f32) - self.center;
        // prevents the initial huge delta which will not be reset and results
        // in unwanted yaw and pitch, if the cursor is outside the window when
        // the program starts.
        if self.delta.x > self.center.x || self.delta.y > self.center.y {
            self.delta = Vector2::new(0.0, 0.0);
        }
    }

    pub fn update_center(&mut self, x: i32, y: i32) {
        self.center = Point2::new(x as f32, y as f32);
    }

    pub fn update_scroll(&mut self, scroll: f32) {
        self.scroll = scroll;
    }

    pub fn drain_scroll(&mut self) -> f32 {
        let s = self.scroll;
        self.scroll = 0.0;
        s
    }

    pub fn delta(&self) -> Vector2<f32> {
        self.delta
    }
}

pub struct Context<'w> {
    pub window: &'w GlWindow,
    pub key_state: KeyState,
    pub mouse_state: MouseState,
    pub screen_width: i32,
    pub screen_height: i32,
    mouse_reset: bool,
    pub render_target: RenderTargetView<R, render::ColorFormat>,
    pub depth_stencil: DepthStencilView<R, render::DepthFormat>,
    pub running: bool,
}

impl<'w> Context<'w> {
    pub fn new(
        window: &'w GlWindow,
        screen_width: i32,
        screen_height: i32,
        render_target: RenderTargetView<R, render::ColorFormat>,
        depth_stencil: DepthStencilView<R, render::DepthFormat>,
    ) -> Context<'w> {
        let mut ctx = Context {
            window,
            key_state: KeyState::new(),
            mouse_state: MouseState::new(),
            screen_width,
            screen_height,
            mouse_reset: true,
            render_target,
            depth_stencil,
            running: true,
        };
        ctx.reset_mouse_pos();
        ctx
    }

    pub fn update_mouse_pos(&mut self, x: i32, y: i32) {
        if self.mouse_reset {
            if x == self.screen_width / 2 && y == self.screen_height / 2 {
                self.mouse_reset = false;
                println!("> reset done");
            } else {
                self.reset_mouse_pos();
                println!("  ({}, {}) resetting mouse...", x, y);
            }
        } else {
            self.mouse_state.update_position(x, y);
        }
        self.window
            .set_cursor_position(self.screen_width / 2, self.screen_height / 2)
            .unwrap();
    }

    pub fn reset_mouse_pos(&mut self) {
        self.mouse_state.update_position(
            self.screen_width / 2,
            self.screen_height / 2,
        );
    }

    pub fn update_dimensions(&mut self, width: u32, height: u32) {
        self.screen_width = width as i32;
        self.screen_height = height as i32;
        self.mouse_state.update_center(
            self.screen_width / 2,
            self.screen_height / 2,
        );
        println!("> resized: {} x {}", width, height);
    }

    pub fn focused(&mut self) {
        self.mouse_reset = true;
        println!("> mouse will be reset");
    }

    pub fn mouse_entered(&mut self) {
        self.window
            .set_cursor_position(self.screen_width / 2, self.screen_height / 2)
            .unwrap();
    }
}
