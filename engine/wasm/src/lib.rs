//! WASM bindings: editor owns scene, camera, selection, and wgpu renderer.

use engine_core::{Camera, Color, NodeId, Vec2};
use engine_input::FrameInput;
use engine_renderer::{meta_list, Renderer};
use engine_scene::Scene;
use engine_selection::{hit_test, SelectionState};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

#[wasm_bindgen(start)]
pub fn wasm_start() {
    console_error_panic_hook::set_once();
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiNode {
    pub id: u32,
    pub name: String,
    pub layer_type: String,
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
    pub shader_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiShaderInfo {
    pub id: u32,
    pub name: String,
    pub category: String,
    pub preview: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiSnapshot {
    pub layers: Vec<UiNode>,
    pub selection: Vec<u32>,
    pub zoom: f32,
    pub camera_x: f32,
    pub camera_y: f32,
    pub needs_redraw: bool,
    pub shaders: Vec<UiShaderInfo>,
}

enum Interaction {
    None,
    Panning { last_screen: Vec2 },
    Dragging { last_world: Vec2 },
}

#[wasm_bindgen]
pub struct Engine {
    scene: Scene,
    camera: Camera,
    selection: SelectionState,
    renderer: Option<Renderer>,
    interaction: Interaction,
    space_held: bool,
    ui_dirty: bool,
    needs_render: bool,
    rect_counter: u32,
    frame_counter: u32,
    shader_counter: u32,
    time: f32,
    pointer_world: Vec2,
}

#[wasm_bindgen]
impl Engine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Engine {
        Engine {
            scene: Scene::new(),
            camera: Camera::default(),
            selection: SelectionState::default(),
            renderer: None,
            interaction: Interaction::None,
            space_held: false,
            ui_dirty: true,
            needs_render: true,
            rect_counter: 0,
            frame_counter: 0,
            shader_counter: 0,
            time: 0.0,
            pointer_world: Vec2::ZERO,
        }
    }

    #[wasm_bindgen]
    pub async fn init(&mut self, canvas: HtmlCanvasElement) -> Result<(), JsValue> {
        let width = canvas.width().max(1);
        let height = canvas.height().max(1);
        self.camera.set_viewport(width as f32, height as f32);

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::BROWSER_WEBGPU | wgpu::Backends::GL,
            ..Default::default()
        });

        let surface = instance
            .create_surface(wgpu::SurfaceTarget::Canvas(canvas))
            .map_err(|e| JsValue::from_str(&format!("surface: {e}")))?;

        let renderer = Renderer::new(instance, surface, width, height)
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        self.renderer = Some(renderer);

        self.create_rectangle(-200.0, -120.0, 400.0, 240.0, 0xFFFFFF);
        // Soft default corner radius for Paper-like starter shape.
        if let Some(id) = self.selection.primary() {
            self.scene.set_radius(id, 8.0);
        }
        self.selection.clear();
        self.needs_render = true;
        self.ui_dirty = true;
        Ok(())
    }

    #[wasm_bindgen]
    pub fn resize(&mut self, css_w: f32, css_h: f32, pixel_w: u32, pixel_h: u32) {
        self.camera.set_viewport(css_w, css_h);
        if let Some(r) = self.renderer.as_mut() {
            r.resize(pixel_w.max(1), pixel_h.max(1));
        }
        self.needs_render = true;
    }

    #[wasm_bindgen]
    pub fn create_rectangle(&mut self, x: f32, y: f32, w: f32, h: f32, fill_hex: u32) -> u32 {
        self.rect_counter += 1;
        let name = format!("Rectangle {}", self.rect_counter);
        let id = self.scene.create_rect(
            name,
            Vec2::new(x, y),
            Vec2::new(w, h),
            Color::from_hex(fill_hex),
        );
        self.selection.set(id);
        self.ui_dirty = true;
        self.needs_render = true;
        id.0
    }

    #[wasm_bindgen]
    pub fn create_frame(&mut self, x: f32, y: f32, w: f32, h: f32) -> u32 {
        self.frame_counter += 1;
        let name = format!("Frame {}", self.frame_counter);
        let id = self
            .scene
            .create_frame(name, Vec2::new(x, y), Vec2::new(w, h));
        self.selection.set(id);
        self.ui_dirty = true;
        self.needs_render = true;
        id.0
    }

    #[wasm_bindgen]
    pub fn create_shader_layer(&mut self, shader_id: u32, x: f32, y: f32, w: f32, h: f32) -> u32 {
        let meta = meta_list()
            .into_iter()
            .find(|m| m.id == shader_id)
            .map(|m| m.name.to_string())
            .unwrap_or_else(|| format!("Shader {shader_id}"));
        self.shader_counter += 1;
        let name = format!("{meta}");
        let id = self
            .scene
            .create_shader(name, Vec2::new(x, y), Vec2::new(w, h), shader_id);
        self.selection.set(id);
        self.ui_dirty = true;
        self.needs_render = true;
        id.0
    }

    #[wasm_bindgen]
    pub fn clear_selection(&mut self) {
        self.selection.clear();
        self.ui_dirty = true;
        self.needs_render = true;
    }

    #[wasm_bindgen]
    pub fn zoom_by(&mut self, factor: f32) {
        let factor = factor.clamp(0.05, 20.0);
        self.camera.zoom_by(factor);
        self.ui_dirty = true;
        self.needs_render = true;
    }

    #[wasm_bindgen]
    pub fn reset_camera(&mut self) {
        self.camera.reset_view();
        self.ui_dirty = true;
        self.needs_render = true;
    }

    #[wasm_bindgen]
    pub fn set_selected_property(&mut self, key: &str, value: f32) {
        let Some(id) = self.selection.primary() else {
            return;
        };
        match key {
            "x" => {
                let y = self.scene.transforms[id.index()].position.y;
                self.scene.set_position(id, Vec2::new(value, y));
            }
            "y" => {
                let x = self.scene.transforms[id.index()].position.x;
                self.scene.set_position(id, Vec2::new(x, value));
            }
            "width" => {
                let h = self.scene.sizes[id.index()].y;
                self.scene.set_size(id, Vec2::new(value.max(1.0), h));
            }
            "height" => {
                let w = self.scene.sizes[id.index()].x;
                self.scene.set_size(id, Vec2::new(w, value.max(1.0)));
            }
            "radius" => self.scene.set_radius(id, value),
            "opacity" => self.scene.set_opacity(id, value),
            _ => {}
        }
        self.ui_dirty = true;
        self.needs_render = true;
    }

    #[wasm_bindgen]
    pub fn set_selected_fill(&mut self, hex: u32) {
        let Some(id) = self.selection.primary() else {
            return;
        };
        self.scene.set_fill(id, Color::from_hex(hex));
        self.ui_dirty = true;
        self.needs_render = true;
    }

    #[wasm_bindgen]
    pub fn select_node(&mut self, id: u32, additive: bool) {
        let nid = NodeId(id);
        if !self.scene.is_alive(nid) {
            return;
        }
        if additive {
            self.selection.toggle(nid);
        } else {
            self.selection.set(nid);
        }
        self.ui_dirty = true;
        self.needs_render = true;
    }

    #[wasm_bindgen]
    pub fn set_visibility(&mut self, id: u32, visible: bool) {
        self.scene.set_visibility(NodeId(id), visible);
        self.ui_dirty = true;
        self.needs_render = true;
    }

    #[wasm_bindgen]
    pub fn set_locked(&mut self, id: u32, locked: bool) {
        self.scene.set_locked(NodeId(id), locked);
        self.ui_dirty = true;
    }

    #[wasm_bindgen]
    pub fn frame(&mut self, input_js: JsValue) -> Result<JsValue, JsValue> {
        let input: FrameInput = serde_wasm_bindgen::from_value(input_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        self.space_held = input.space;
        self.pointer_world = self.camera.screen_to_world(input.pointer());
        self.handle_input(&input);

        // Animate when visible shader layers exist.
        if self.scene.has_visible_shaders() {
            self.time += 1.0 / 60.0;
            self.needs_render = true;
        }

        self.scene.update_world_transforms();

        if self.needs_render {
            if let Some(renderer) = self.renderer.as_mut() {
                renderer
                    .render_frame(
                        &self.scene,
                        &self.selection.selected,
                        &self.camera,
                        self.time,
                        self.pointer_world,
                    )
                    .map_err(|e| JsValue::from_str(&e.to_string()))?;
            }
            if !self.scene.has_visible_shaders() {
                self.needs_render = false;
            }
            self.scene.instance_dirty = false;
        }

        let mut snap = self.build_snapshot();
        snap.needs_redraw = self.ui_dirty;
        self.ui_dirty = false;
        serde_wasm_bindgen::to_value(&snap).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen]
    pub fn get_ui_snapshot(&self) -> Result<JsValue, JsValue> {
        let snap = self.build_snapshot();
        serde_wasm_bindgen::to_value(&snap).map_err(|e| JsValue::from_str(&e.to_string()))
    }
}

impl Engine {
    fn build_snapshot(&self) -> UiSnapshot {
        let metas = meta_list();
        let shaders = metas
            .iter()
            .map(|m| UiShaderInfo {
                id: m.id,
                name: m.name.to_string(),
                category: m.category.to_string(),
                preview: m.preview.to_string(),
            })
            .collect();

        let layers = self
            .scene
            .layer_list()
            .into_iter()
            .map(|n| {
                let shader_name = if n.layer_type == engine_scene::LayerType::Shader {
                    metas
                        .iter()
                        .find(|m| m.id == n.shader_id)
                        .map(|m| m.name.to_string())
                        .unwrap_or_default()
                } else {
                    String::new()
                };
                UiNode {
                    id: n.id.0,
                    name: n.name,
                    layer_type: match n.layer_type {
                        engine_scene::LayerType::Rectangle => "Rectangle".into(),
                        engine_scene::LayerType::Group => "Group".into(),
                        engine_scene::LayerType::Text => "Text".into(),
                        engine_scene::LayerType::Image => "Image".into(),
                        engine_scene::LayerType::Shader => "Shader".into(),
                        engine_scene::LayerType::Frame => "Frame".into(),
                    },
                    visible: n.visible,
                    locked: n.locked,
                    x: n.x,
                    y: n.y,
                    width: n.width,
                    height: n.height,
                    fill: n.fill,
                    radius: n.radius,
                    opacity: n.opacity,
                    shader_id: n.shader_id,
                    shader_name,
                }
            })
            .collect();

        UiSnapshot {
            layers,
            selection: self.selection.selected.iter().map(|id| id.0).collect(),
            zoom: self.camera.zoom,
            camera_x: self.camera.position.x,
            camera_y: self.camera.position.y,
            needs_redraw: self.ui_dirty,
            shaders,
        }
    }

    fn handle_input(&mut self, input: &FrameInput) {
        let pointer = input.pointer();

        if input.wheel_delta_y.abs() > 0.0 && (input.ctrl || input.meta) {
            let factor = if input.wheel_delta_y < 0.0 {
                1.1
            } else {
                1.0 / 1.1
            };
            self.camera.zoom_at(pointer, factor);
            self.needs_render = true;
            self.ui_dirty = true;
        }

        if input.pointer_pressed {
            let pan_gesture = input.is_middle() || (input.is_left() && self.space_held);
            if pan_gesture {
                self.interaction = Interaction::Panning {
                    last_screen: pointer,
                };
            } else if input.is_left() {
                let world = self.camera.screen_to_world(pointer);
                if let Some(hit) = hit_test(&self.scene, world) {
                    if input.shift {
                        self.selection.toggle(hit);
                    } else if !self.selection.contains(hit) {
                        self.selection.set(hit);
                    }
                    if !self.scene.is_locked(hit) {
                        self.interaction = Interaction::Dragging { last_world: world };
                    }
                    self.ui_dirty = true;
                    self.needs_render = true;
                } else {
                    if !input.shift {
                        self.selection.clear();
                        self.ui_dirty = true;
                        self.needs_render = true;
                    }
                    self.interaction = Interaction::None;
                }
            }
        }

        if input.pointer_down {
            match &self.interaction {
                Interaction::Panning { last_screen } => {
                    let delta = pointer - *last_screen;
                    if delta.length_squared() > 0.0 {
                        self.camera.pan(delta);
                        self.interaction = Interaction::Panning {
                            last_screen: pointer,
                        };
                        self.needs_render = true;
                        self.ui_dirty = true;
                    }
                }
                Interaction::Dragging { last_world } => {
                    let world = self.camera.screen_to_world(pointer);
                    let delta = world - *last_world;
                    if delta.length_squared() > 0.0 {
                        let ids: Vec<NodeId> = self.selection.selected.clone();
                        for id in ids {
                            if !self.scene.is_locked(id) {
                                self.scene.translate(id, delta);
                            }
                        }
                        self.interaction = Interaction::Dragging { last_world: world };
                        self.needs_render = true;
                        self.ui_dirty = true;
                    }
                }
                Interaction::None => {}
            }
        }

        if input.pointer_released {
            self.interaction = Interaction::None;
        }
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}
