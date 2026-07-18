use crate::rect_pipeline::RectPipeline;
use crate::shader_pipeline::{ShaderDrawUniforms, ShaderPipelines, DRAW_UNIFORM_ALIGN};
use crate::uniforms::{RectInstance, ViewUniforms};
use engine_core::{Camera, NodeId, Vec2};
use engine_scene::{LayerType, Scene};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RendererError {
    #[error("failed to create surface: {0}")]
    Surface(String),
    #[error("failed to request adapter")]
    Adapter,
    #[error("failed to request device: {0}")]
    Device(String),
    #[error("surface error: {0}")]
    SurfaceTexture(#[from] wgpu::SurfaceError),
}

pub struct Renderer {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface: wgpu::Surface<'static>,
    pub config: wgpu::SurfaceConfiguration,
    pub rect_pipeline: RectPipeline,
    pub shader_pipelines: ShaderPipelines,
    pub clear_color: wgpu::Color,
    sample_count: u32,
    msaa_texture: Option<wgpu::Texture>,
    msaa_view: Option<wgpu::TextureView>,
    width: u32,
    height: u32,
}

impl Renderer {
    pub async fn new(
        instance: wgpu::Instance,
        surface: wgpu::Surface<'static>,
        width: u32,
        height: u32,
    ) -> Result<Self, RendererError> {
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or(RendererError::Adapter)?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("canvas_device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                        .using_resolution(adapter.limits()),
                    memory_hints: Default::default(),
                },
                None,
            )
            .await
            .map_err(|e| RendererError::Device(e.to_string()))?;

        let caps = surface.get_capabilities(&adapter);
        let format = caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(caps.formats[0]);

        let sample_count = if adapter
            .get_texture_format_features(format)
            .flags
            .contains(wgpu::TextureFormatFeatureFlags::MULTISAMPLE_X4)
        {
            4
        } else {
            1
        };

        let width = width.max(1);
        let height = height.max(1);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width,
            height,
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let rect_pipeline = RectPipeline::new(&device, format, sample_count);
        let shader_pipelines = ShaderPipelines::new(&device, format, sample_count);

        let (msaa_texture, msaa_view) = if sample_count > 1 {
            let (tex, view) = create_msaa(&device, format, width, height, sample_count);
            (Some(tex), Some(view))
        } else {
            (None, None)
        };

        // Warm paper desk gray (#d2d2ce)
        Ok(Self {
            device,
            queue,
            surface,
            config,
            rect_pipeline,
            shader_pipelines,
            clear_color: wgpu::Color {
                r: 212.0 / 255.0,
                g: 212.0 / 255.0,
                b: 208.0 / 255.0,
                a: 1.0,
            },
            sample_count,
            msaa_texture,
            msaa_view,
            width,
            height,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        let width = width.max(1);
        let height = height.max(1);
        if self.width == width && self.height == height {
            return;
        }
        self.width = width;
        self.height = height;
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
        if self.sample_count > 1 {
            let (tex, view) = create_msaa(
                &self.device,
                self.config.format,
                width,
                height,
                self.sample_count,
            );
            self.msaa_texture = Some(tex);
            self.msaa_view = Some(view);
        }
    }

    pub fn shader_count(&self) -> usize {
        self.shader_pipelines.metas.len()
    }

    pub fn shader_meta(&self, id: u32) -> Option<&crate::shader_lib::ShaderMeta> {
        self.shader_pipelines.metas.get(id as usize)
    }

    pub fn render_frame(
        &mut self,
        scene: &Scene,
        selected: &[NodeId],
        camera: &Camera,
        time: f32,
        mouse_world: Vec2,
    ) -> Result<(), RendererError> {
        let view = ViewUniforms::from_camera(camera);
        self.queue.write_buffer(
            &self.rect_pipeline.view_buffer,
            0,
            bytemuck::bytes_of(&view),
        );
        self.queue.write_buffer(
            &self.shader_pipelines.view_buffer,
            0,
            bytemuck::bytes_of(&view),
        );

        enum DrawCmd {
            Rect { instance: u32, selected: bool },
            Shader {
                shader_id: u32,
                slot: u32,
                selected: bool,
            },
        }

        let mut rect_instances: Vec<RectInstance> = Vec::new();
        let mut shader_draws: Vec<ShaderDrawUniforms> = Vec::new();
        let mut cmds: Vec<DrawCmd> = Vec::new();

        for id in scene.paint_order() {
            let i = id.index();
            let selected_flag = selected.contains(&id);
            match scene.layer_types[i] {
                LayerType::Shader => {
                    let shader_id = scene.shader_ids[i];
                    if self.shader_pipelines.pipeline_for(shader_id).is_none() {
                        continue;
                    }
                    let t = &scene.transforms[i];
                    let size = scene.sizes[i];
                    let local_mouse = Vec2::new(
                        ((mouse_world.x - t.position.x) / size.x.max(1.0)).clamp(0.0, 1.0),
                        ((mouse_world.y - t.position.y) / size.y.max(1.0)).clamp(0.0, 1.0),
                    );
                    let slot = shader_draws.len() as u32;
                    shader_draws.push(ShaderDrawUniforms {
                        rect: [t.position.x, t.position.y, size.x, size.y],
                        time_opacity: [time, scene.opacities[i], 0.0, 0.0],
                        mouse: [local_mouse.x, local_mouse.y, 0.0, 0.0],
                    });
                    cmds.push(DrawCmd::Shader {
                        shader_id,
                        slot,
                        selected: selected_flag,
                    });
                }
                LayerType::Rectangle | LayerType::Frame => {
                    let t = &scene.transforms[i];
                    let size = scene.sizes[i];
                    let fill = scene.fills[i];
                    let instance = rect_instances.len() as u32;
                    rect_instances.push(RectInstance {
                        rect: [t.position.x, t.position.y, size.x, size.y],
                        color: fill.to_array(),
                        params: [
                            scene.radii[i],
                            scene.opacities[i],
                            if selected_flag { 1.0 } else { 0.0 },
                            0.0,
                        ],
                    });
                    cmds.push(DrawCmd::Rect {
                        instance,
                        selected: selected_flag,
                    });
                }
                _ => {}
            }
        }

        self.rect_pipeline
            .upload_instances(&self.device, &self.queue, &rect_instances);

        if !shader_draws.is_empty() {
            let mut bytes = vec![0u8; shader_draws.len() * DRAW_UNIFORM_ALIGN as usize];
            for (i, draw) in shader_draws.iter().enumerate() {
                let offset = i * DRAW_UNIFORM_ALIGN as usize;
                let src = bytemuck::bytes_of(draw);
                bytes[offset..offset + src.len()].copy_from_slice(src);
            }
            self.shader_pipelines
                .ensure_draw_capacity(&self.device, shader_draws.len());
            self.queue
                .write_buffer(&self.shader_pipelines.draw_buffer, 0, &bytes);
        }

        let frame = self.surface.get_current_texture()?;
        let resolve_view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("frame_encoder"),
            });

        {
            let (attach_view, resolve_target) = if let Some(msaa) = self.msaa_view.as_ref() {
                (msaa, Some(&resolve_view))
            } else {
                (&resolve_view, None)
            };

            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("main_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: attach_view,
                    resolve_target,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.clear_color),
                        store: if resolve_target.is_some() {
                            wgpu::StoreOp::Discard
                        } else {
                            wgpu::StoreOp::Store
                        },
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            for cmd in &cmds {
                match *cmd {
                    DrawCmd::Rect { instance, selected } => {
                        pass.set_pipeline(&self.rect_pipeline.pipeline);
                        pass.set_bind_group(0, &self.rect_pipeline.bind_group, &[]);
                        pass.set_vertex_buffer(0, self.rect_pipeline.vertex_buffer.slice(..));
                        pass.set_vertex_buffer(1, self.rect_pipeline.instance_buffer.slice(..));
                        pass.draw(0..6, instance..instance + 1);
                        if selected {
                            pass.set_pipeline(&self.rect_pipeline.outline_pipeline);
                            pass.draw(0..6, instance..instance + 1);
                        }
                    }
                    DrawCmd::Shader {
                        shader_id,
                        slot,
                        selected,
                    } => {
                        let Some(pipeline) = self.shader_pipelines.pipeline_for(shader_id) else {
                            continue;
                        };
                        let offset = slot as u32 * DRAW_UNIFORM_ALIGN as u32;
                        pass.set_pipeline(pipeline);
                        pass.set_bind_group(0, &self.shader_pipelines.bind_group, &[offset]);
                        pass.set_vertex_buffer(0, self.shader_pipelines.vertex_buffer.slice(..));
                        pass.draw(0..6, 0..1);
                        if selected {
                            pass.set_pipeline(&self.shader_pipelines.outline_pipeline);
                            pass.set_bind_group(0, &self.shader_pipelines.bind_group, &[offset]);
                            pass.draw(0..6, 0..1);
                        }
                    }
                }
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        frame.present();
        Ok(())
    }
}

fn create_msaa(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
    width: u32,
    height: u32,
    sample_count: u32,
) -> (wgpu::Texture, wgpu::TextureView) {
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("msaa_color"),
        size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });
    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    (texture, view)
}
