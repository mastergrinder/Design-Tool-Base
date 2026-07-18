use crate::shader_lib::{self, ShaderMeta};
use crate::uniforms::ViewUniforms;
use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

pub const DRAW_UNIFORM_ALIGN: u64 = 256;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct ShaderDrawUniforms {
    pub rect: [f32; 4],
    pub time_opacity: [f32; 4],
    pub mouse: [f32; 4],
}

pub struct ShaderPipelines {
    pub pipelines: Vec<wgpu::RenderPipeline>,
    pub metas: Vec<ShaderMeta>,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub view_buffer: wgpu::Buffer,
    pub draw_buffer: wgpu::Buffer,
    pub draw_capacity: usize,
    pub bind_group: wgpu::BindGroup,
    pub vertex_buffer: wgpu::Buffer,
    pub outline_pipeline: wgpu::RenderPipeline,
}

impl ShaderPipelines {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat, sample_count: u32) -> Self {
        let sources = shader_lib::compiled_sources();
        let metas: Vec<ShaderMeta> = sources.iter().map(|(m, _)| *m).collect();

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("shader_bgl"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: true,
                        min_binding_size: wgpu::BufferSize::new(std::mem::size_of::<
                            ShaderDrawUniforms,
                        >() as u64),
                    },
                    count: None,
                },
            ],
        });

        let view_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("shader_view"),
            size: std::mem::size_of::<ViewUniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let draw_capacity = 16;
        let draw_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("shader_draw"),
            size: draw_capacity as u64 * DRAW_UNIFORM_ALIGN,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group =
            Self::make_bind_group(device, &bind_group_layout, &view_buffer, &draw_buffer);

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("shader_pl"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let vertex_layout = [wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<[f32; 2]>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32x2],
        }];

        let mut pipelines = Vec::with_capacity(sources.len());
        for (meta, source) in &sources {
            let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(meta.name),
                source: wgpu::ShaderSource::Wgsl(source.as_str().into()),
            });
            let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some(meta.name),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &module,
                    entry_point: Some("vs_main"),
                    compilation_options: Default::default(),
                    buffers: &vertex_layout,
                },
                fragment: Some(wgpu::FragmentState {
                    module: &module,
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
                multisample: wgpu::MultisampleState {
                    count: sample_count,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
                cache: None,
            });
            pipelines.push(pipeline);
        }

        let outline_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("shader_outline"),
            source: wgpu::ShaderSource::Wgsl(shader_lib::outline_source().into()),
        });
        let outline_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("shader_outline_pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &outline_module,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &vertex_layout,
            },
            fragment: Some(wgpu::FragmentState {
                module: &outline_module,
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
            multisample: wgpu::MultisampleState {
                count: sample_count,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        let quad: [[f32; 2]; 6] = [
            [0.0, 0.0],
            [1.0, 0.0],
            [1.0, 1.0],
            [0.0, 0.0],
            [1.0, 1.0],
            [0.0, 1.0],
        ];
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("shader_quad"),
            contents: bytemuck::cast_slice(&quad),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Self {
            pipelines,
            metas,
            bind_group_layout,
            view_buffer,
            draw_buffer,
            draw_capacity,
            bind_group,
            vertex_buffer,
            outline_pipeline,
        }
    }

    fn make_bind_group(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        view_buffer: &wgpu::Buffer,
        draw_buffer: &wgpu::Buffer,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("shader_bg"),
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: view_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: draw_buffer,
                        offset: 0,
                        size: wgpu::BufferSize::new(
                            std::mem::size_of::<ShaderDrawUniforms>() as u64
                        ),
                    }),
                },
            ],
        })
    }

    pub fn ensure_draw_capacity(&mut self, device: &wgpu::Device, count: usize) {
        if count <= self.draw_capacity {
            return;
        }
        let mut cap = self.draw_capacity;
        while cap < count {
            cap *= 2;
        }
        self.draw_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("shader_draw"),
            size: cap as u64 * DRAW_UNIFORM_ALIGN,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        self.draw_capacity = cap;
        self.bind_group = Self::make_bind_group(
            device,
            &self.bind_group_layout,
            &self.view_buffer,
            &self.draw_buffer,
        );
    }

    pub fn rebuild_bind_group(&mut self, device: &wgpu::Device) {
        self.bind_group = Self::make_bind_group(
            device,
            &self.bind_group_layout,
            &self.view_buffer,
            &self.draw_buffer,
        );
    }

    pub fn pipeline_for(&self, shader_id: u32) -> Option<&wgpu::RenderPipeline> {
        self.pipelines.get(shader_id as usize)
    }
}
