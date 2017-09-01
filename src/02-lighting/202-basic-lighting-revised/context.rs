use std::collections::HashSet;
use cgmath::{Point2, Vector2};
use glutin::{GlWindow, VirtualKeyCode};


pub struct KeyState {
    pressed: HashSet<VirtualKeyCode>,
}

impl KeyState {
    pub fn new() -> KeyState {
        KeyState {
            pressed: HashSet::new(),
        }
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

    pub fn update_position(&mut self, x: f32, y: f32) {
        self.delta = Point2::new(x, y) - self.center;
    }

    pub fn update_center(&mut self, x: f32, y: f32) {
        self.center = Point2::new(x, y);
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

pub struct Context {
    pub key_state: KeyState,
    pub mouse_state: MouseState,
    pub screen_width: f32,
    pub screen_height: f32,
    mouse_reset: bool,
}

impl Context {
    pub fn new(screen_width: f32, screen_height: f32) -> Context {
        let mut ctx = Context {
            key_state: KeyState::new(),
            mouse_state: MouseState::new(),
            screen_width,
            screen_height,
            mouse_reset: true,
        };
        ctx.reset_mouse_pos();
        ctx
    }

    pub fn update_mouse_pos(&mut self, window: &GlWindow, x: f32, y: f32) {
        if self.mouse_reset {
            if relative_eq!(x, self.screen_width / 2.0) && relative_eq!(y, self.screen_height / 2.0)
            {
                self.mouse_reset = false;
            } else {
                self.reset_mouse_pos();
            }
        } else {
            self.mouse_state.update_position(x, y);
        }
        window
            .set_cursor_position(self.screen_width as i32 / 2, self.screen_height as i32 / 2)
            .unwrap();
    }

    pub fn reset_mouse_pos(&mut self) {
        self.mouse_state
            .update_position(self.screen_width / 2.0, self.screen_height / 2.0);
    }

    pub fn update_dimensions(&mut self, width: f32, height: f32) {
        self.screen_width = width;
        self.screen_height = height;
        self.mouse_state.update_center(width / 2.0, height / 2.0);
    }

    pub fn focused(&mut self) {
        self.mouse_reset = true;
    }

    pub fn mouse_entered(&mut self, window: &GlWindow) {
        window
            .set_cursor_position(self.screen_width as i32 / 2, self.screen_height as i32 / 2)
            .unwrap();
    }
}
