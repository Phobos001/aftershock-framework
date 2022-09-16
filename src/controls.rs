use device_query::{DeviceQuery, DeviceState, MouseState, Keycode};
use crate::vector2::*;

pub enum MouseButton {
    None,
    Left,
    Right,
    Middle,
    X1,
    X2,
}

#[derive(Debug, Clone, Copy)]
pub struct KeyBind {
    pub keybit: u8,
    pub keycode: Keycode,
}

pub struct ControlData {
    pub binds: Vec<KeyBind>,
    pub controls: u128,
    pub controls_last: u128,

    pub device_state: DeviceState,

    pub mouse: Vector2,
    pub mouse_delta: Vector2,
}

impl ControlData {
    pub const CONSOLE: u8       = 122;
    pub const MOUSE_LEFT: u8    = 123;
    pub const MOUSE_RIGHT: u8   = 124;
    pub const MOUSE_MIDDLE: u8  = 125;
    pub const MOUSE_X1: u8      = 126;
    pub const MOUSE_X2: u8      = 127;

    pub fn new() -> ControlData {
        ControlData {
            binds: vec![KeyBind { keybit: ControlData::CONSOLE, keycode: Keycode::Grave }; 1],
            controls: 0,
            controls_last: 0,

            device_state: DeviceState::new(),

            mouse: Vector2::ZERO,
            mouse_delta: Vector2::ZERO,
        }
    }

    pub fn is_control_down(&self, control: u8) -> bool {
        return self.controls & (1 << control) != 0;
    }

    pub fn is_control_pressed(&self, control: u8) -> bool {
        !(self.controls_last & (1 << control) != 0) && (self.controls & (1 << control) != 0)
    }

    pub fn is_control_released(&self, control: u8) -> bool {
        (self.controls_last & (1 << control) != 0) && !(self.controls & (1 << control) != 0)
    }

    pub fn update_mouse_delta(&mut self, xrel: f64, yrel: f64) {
        self.mouse_delta = Vector2::new(xrel as f64, yrel as f64);
    }

    pub fn update_controls(&mut self, screen_width: usize, screen_height: usize, video_width: usize, video_height: usize, fullscreen: bool, sdl_x: f64, sdl_y: f64) {
        let mouse_state: MouseState = self.device_state.get_mouse();

        if fullscreen {
            let width_mul = video_width as f64 / screen_width as f64;
            let height_mul = video_height as f64 / screen_height as f64;

            self.mouse = Vector2::new(mouse_state.coords.0 as f64 * width_mul, mouse_state.coords.1 as f64 * height_mul);
            //self.mouse = Vector2::new(sdl_x, sdl_y);
        } else {
            self.mouse = Vector2::new(sdl_x, sdl_y);
        }
        

        if mouse_state.button_pressed[0] {
            self.controls |= 1 << ControlData::MOUSE_LEFT;
        }

        if mouse_state.button_pressed[1] {
            self.controls |= 1 << ControlData::MOUSE_RIGHT;
        }

        if mouse_state.button_pressed[2] {
            self.controls |= 1 << ControlData::MOUSE_MIDDLE;
        }

        if mouse_state.button_pressed[3] {
            self.controls |= 1 << ControlData::MOUSE_X1;
        }

        if mouse_state.button_pressed[4] {
            self.controls |= 1 << ControlData::MOUSE_X2;
        }

        let keys: Vec<Keycode> = self.device_state.get_keys();

        self.controls_last = self.controls;
        self.controls = 0;
        
        for key in keys.iter() {
            for bind in &self.binds {
                if key == &bind.keycode {
                    self.controls |= 1 << bind.keybit;
                }
            }
        }
    }
}