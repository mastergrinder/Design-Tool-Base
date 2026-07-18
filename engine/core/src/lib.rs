//! Core math, camera, and coordinate conversions.

mod camera;
mod color;
mod dirty;
mod node_id;
mod rect;

pub use camera::Camera;
pub use color::Color;
pub use dirty::DirtyFlags;
pub use node_id::NodeId;
pub use rect::Rect;

pub type Mat3 = glam::Mat3;
pub type Vec2 = glam::Vec2;
