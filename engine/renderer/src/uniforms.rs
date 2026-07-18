use bytemuck::{Pod, Zeroable};
use engine_core::Mat3;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct ViewUniforms {
    /// Column-major 3x3 stored as 3 vec4s (16-byte aligned) for WGSL.
    pub view_proj: [[f32; 4]; 3],
    pub viewport: [f32; 4],
}

impl ViewUniforms {
    pub fn from_camera(camera: &engine_core::Camera) -> Self {
        let m = camera.view_proj_matrix();
        Self {
            view_proj: mat3_to_cols(m),
            viewport: [
                camera.viewport_size.x,
                camera.viewport_size.y,
                camera.zoom,
                0.0,
            ],
        }
    }
}

fn mat3_to_cols(m: Mat3) -> [[f32; 4]; 3] {
    let c0 = m.x_axis;
    let c1 = m.y_axis;
    let c2 = m.z_axis;
    [
        [c0.x, c0.y, c0.z, 0.0],
        [c1.x, c1.y, c1.z, 0.0],
        [c2.x, c2.y, c2.z, 0.0],
    ]
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct RectInstance {
    /// World-space center xy, size zw
    pub rect: [f32; 4],
    pub color: [f32; 4],
    /// radius, opacity, selected flag, pad
    pub params: [f32; 4],
}
