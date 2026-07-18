//! Normalized input events from the browser.

use engine_core::Vec2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum PointerButton {
    Left = 0,
    Middle = 1,
    Right = 2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameInput {
    pub pointer_x: f32,
    pub pointer_y: f32,
    pub pointer_down: bool,
    pub pointer_pressed: bool,
    pub pointer_released: bool,
    pub button: u8,
    pub wheel_delta_y: f32,
    pub ctrl: bool,
    pub shift: bool,
    pub space: bool,
    pub meta: bool,
}

impl Default for FrameInput {
    fn default() -> Self {
        Self {
            pointer_x: 0.0,
            pointer_y: 0.0,
            pointer_down: false,
            pointer_pressed: false,
            pointer_released: false,
            button: 0,
            wheel_delta_y: 0.0,
            ctrl: false,
            shift: false,
            space: false,
            meta: false,
        }
    }
}

impl FrameInput {
    pub fn pointer(&self) -> Vec2 {
        Vec2::new(self.pointer_x, self.pointer_y)
    }

    pub fn is_middle(&self) -> bool {
        self.button == PointerButton::Middle as u8
    }

    pub fn is_left(&self) -> bool {
        self.button == PointerButton::Left as u8
    }
}
