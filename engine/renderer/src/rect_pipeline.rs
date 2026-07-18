use crate::uniforms::{RectInstance, ViewUniforms};
use wgpu::util::DeviceExt;

pub struct RectPipeline {
    pub pipeline: wgpu::RenderPipeline,
    pub outline_pipeline: wgpu::RenderPipeline,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
    pub view_buffer: wgpu::Buffer,
    pub vertex_buffer: wgpu::Buffer,
    pub instance_buffer: wgpu::Buffer,
    pub instance_capacity: usize,
    pub instance_count: u32,
}

impl RectPipeline {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("rect_shader"),
            source: wgpu::ShaderSource::Wgsl(RECT_WGSL.into()),
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("view_bgl"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let view_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("view_uniforms"),
            size: std::mem::size_of::<ViewUniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("view_bg"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: view_buffer.as_entire_binding(),
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("rect_pl"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let vertex_buffers = &[
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<[f32; 2]>() as u64,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &wgpu::vertex_attr_array![0 => Float32x2],
            },
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<RectInstance>() as u64,
                step_mode: wgpu::VertexStepMode::Instance,
                attributes: &wgpu::vertex_attr_array![
                    1 => Float32x4,
                    2 => Float32x4,
                    3 => Float32x4,
                ],
            },
        ];

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("rect_pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: vertex_buffers,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let outline_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("outline_pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: vertex_buffers,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_outline"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        // Unit quad: two triangles, corners in [0,1]
        let quad: [[f32; 2]; 6] = [
            [0.0, 0.0],
            [1.0, 0.0],
            [1.0, 1.0],
            [0.0, 0.0],
            [1.0, 1.0],
            [0.0, 1.0],
        ];
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("quad_vb"),
            contents: bytemuck::cast_slice(&quad),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let instance_capacity = 64;
        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("rect_instances"),
            size: (instance_capacity * std::mem::size_of::<RectInstance>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            pipeline,
            outline_pipeline,
            bind_group_layout,
            bind_group,
            view_buffer,
            vertex_buffer,
            instance_buffer,
            instance_capacity,
            instance_count: 0,
        }
    }

    pub fn ensure_instance_capacity(&mut self, device: &wgpu::Device, count: usize) {
        if count <= self.instance_capacity {
            return;
        }
        let mut cap = self.instance_capacity;
        while cap < count {
            cap *= 2;
        }
        self.instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("rect_instances"),
            size: (cap * std::mem::size_of::<RectInstance>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        self.instance_capacity = cap;
    }

    pub fn upload_instances(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        instances: &[RectInstance],
    ) {
        self.ensure_instance_capacity(device, instances.len().max(1));
        if !instances.is_empty() {
            queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(instances));
        }
        self.instance_count = instances.len() as u32;
    }
}

const RECT_WGSL: &str = r#"
struct ViewUniforms {
    view_proj: mat3x3<f32>,
    // packed as 3 x vec4 in Rust; WGSL mat3x3 may differ — use explicit columns
    viewport: vec4<f32>,
}

// Manual layout matching Rust [[f32;4];3] + [f32;4]
struct ViewUniformsRaw {
    c0: vec4<f32>,
    c1: vec4<f32>,
    c2: vec4<f32>,
    viewport: vec4<f32>,
}

@group(0) @binding(0) var<uniform> view: ViewUniformsRaw;

struct VertexInput {
    @location(0) pos: vec2<f32>,
    @location(1) rect: vec4<f32>,
    @location(2) color: vec4<f32>,
    @location(3) params: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) local: vec2<f32>,
    @location(2) size: vec2<f32>,
    @location(3) radius: f32,
    @location(4) selected: f32,
}

fn apply_view(p: vec2<f32>) -> vec4<f32> {
    let m = mat3x3<f32>(view.c0.xyz, view.c1.xyz, view.c2.xyz);
    let clip = m * vec3<f32>(p, 1.0);
    return vec4<f32>(clip.xy, 0.0, 1.0);
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let origin = in.rect.xy;
    let size = in.rect.zw;
    let world = origin + in.pos * size;
    out.clip_pos = apply_view(world);
    out.color = vec4<f32>(in.color.rgb, in.color.a * in.params.y);
    out.local = in.pos * size;
    out.size = size;
    out.radius = in.params.x;
    out.selected = in.params.z;
    return out;
}

fn sd_rounded_box(p: vec2<f32>, b: vec2<f32>, r: f32) -> f32 {
    let q = abs(p) - b + vec2<f32>(r);
    return length(max(q, vec2<f32>(0.0))) + min(max(q.x, q.y), 0.0) - r;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let half = in.size * 0.5;
    let p = in.local - half;
    let r = min(in.radius, min(half.x, half.y));
    let d = sd_rounded_box(p, half, r);
    let aa = max(fwidth(d), 0.001);
    let alpha = 1.0 - smoothstep(-aa, aa, d);
    return vec4<f32>(in.color.rgb, in.color.a * alpha);
}

@fragment
fn fs_outline(in: VertexOutput) -> @location(0) vec4<f32> {
    if (in.selected < 0.5) {
        discard;
    }
    let half = in.size * 0.5;
    let p = in.local - half;
    let r = min(in.radius, min(half.x, half.y));
    let d = sd_rounded_box(p, half, r);
    let thickness = 1.5 / max(view.viewport.z, 0.001);
    let aa = max(fwidth(d), 0.001);
    let outer = 1.0 - smoothstep(-aa, aa, d);
    let inner = 1.0 - smoothstep(-aa, aa, d + thickness);
    let edge = clamp(outer - inner, 0.0, 1.0);
    return vec4<f32>(0.15, 0.45, 0.95, edge);
}
"#;
