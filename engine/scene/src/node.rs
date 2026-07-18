use engine_core::{Color, NodeId, Vec2};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum LayerType {
    Rectangle = 0,
    Group = 1,
    Text = 2,
    Image = 3,
    Shader = 4,
    Frame = 5,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Transform2D {
    pub position: Vec2,
    pub rotation: f32,
    pub scale: Vec2,
}

impl Default for Transform2D {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            rotation: 0.0,
            scale: Vec2::ONE,
        }
    }
}

impl Transform2D {
    pub fn to_matrix(self) -> engine_core::Mat3 {
        let t = engine_core::Mat3::from_translation(self.position);
        let r = engine_core::Mat3::from_angle(self.rotation);
        let s = engine_core::Mat3::from_scale(self.scale);
        t * r * s
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeSnapshot {
    pub id: NodeId,
    pub name: String,
    pub layer_type: LayerType,
    pub visible: bool,
    pub locked: bool,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub fill: [f32; 4],
    pub radius: f32,
    pub opacity: f32,
    pub shader_id: u32,
}

impl NodeSnapshot {
    pub fn fill_color(&self) -> Color {
        Color {
            r: self.fill[0],
            g: self.fill[1],
            b: self.fill[2],
            a: self.fill[3],
        }
    }
}
