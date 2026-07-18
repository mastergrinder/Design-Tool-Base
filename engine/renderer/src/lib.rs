//! WebGPU renderer: device, surface, instanced rectangles, shader layers.

mod gpu;
mod rect_pipeline;
mod shader_lib;
mod shader_pipeline;
mod uniforms;

pub use gpu::{Renderer, RendererError};
pub use shader_lib::{meta_list, ShaderMeta};
pub use uniforms::ViewUniforms;
