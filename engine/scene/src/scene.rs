use crate::{LayerType, NodeSnapshot, Transform2D};
use engine_core::{Color, DirtyFlags, NodeId, Rect, Vec2};

/// Data-oriented scene storage. Hot paths iterate SoA arrays.
pub struct Scene {
    pub names: Vec<String>,
    pub layer_types: Vec<LayerType>,
    pub transforms: Vec<Transform2D>,
    pub world_transforms: Vec<engine_core::Mat3>,
    pub sizes: Vec<Vec2>,
    pub fills: Vec<Color>,
    pub radii: Vec<f32>,
    pub opacities: Vec<f32>,
    pub visibility: Vec<bool>,
    pub locked: Vec<bool>,
    pub parents: Vec<NodeId>,
    pub children: Vec<Vec<NodeId>>,
    pub dirty: Vec<DirtyFlags>,
    pub alive: Vec<bool>,
    /// Built-in shader catalog id; `u32::MAX` when not a shader layer.
    pub shader_ids: Vec<u32>,
    /// Root children (document roots).
    pub roots: Vec<NodeId>,
    pub world_dirty: bool,
    pub instance_dirty: bool,
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}

impl Scene {
    pub fn new() -> Self {
        Self {
            names: Vec::new(),
            layer_types: Vec::new(),
            transforms: Vec::new(),
            world_transforms: Vec::new(),
            sizes: Vec::new(),
            fills: Vec::new(),
            radii: Vec::new(),
            opacities: Vec::new(),
            visibility: Vec::new(),
            locked: Vec::new(),
            parents: Vec::new(),
            children: Vec::new(),
            dirty: Vec::new(),
            alive: Vec::new(),
            shader_ids: Vec::new(),
            roots: Vec::new(),
            world_dirty: true,
            instance_dirty: true,
        }
    }

    pub fn len(&self) -> usize {
        self.alive.len()
    }

    pub fn is_empty(&self) -> bool {
        self.alive.iter().all(|&a| !a)
    }

    fn push_common(
        &mut self,
        name: String,
        layer_type: LayerType,
        position: Vec2,
        size: Vec2,
        fill: Color,
        radius: f32,
        shader_id: u32,
    ) -> NodeId {
        let id = NodeId(self.alive.len() as u32);
        self.names.push(name);
        self.layer_types.push(layer_type);
        self.transforms.push(Transform2D {
            position,
            ..Default::default()
        });
        self.world_transforms.push(engine_core::Mat3::IDENTITY);
        self.sizes.push(size);
        self.fills.push(fill);
        self.radii.push(radius);
        self.opacities.push(1.0);
        self.visibility.push(true);
        self.locked.push(false);
        self.parents.push(NodeId::NONE);
        self.children.push(Vec::new());
        self.dirty.push(DirtyFlags::ALL);
        self.alive.push(true);
        self.shader_ids.push(shader_id);
        self.roots.push(id);
        self.world_dirty = true;
        self.instance_dirty = true;
        id
    }

    pub fn create_rect(
        &mut self,
        name: impl Into<String>,
        position: Vec2,
        size: Vec2,
        fill: Color,
    ) -> NodeId {
        self.push_common(
            name.into(),
            LayerType::Rectangle,
            position,
            size,
            fill,
            0.0,
            u32::MAX,
        )
    }

    pub fn create_frame(&mut self, name: impl Into<String>, position: Vec2, size: Vec2) -> NodeId {
        self.push_common(
            name.into(),
            LayerType::Frame,
            position,
            size,
            Color::rgba(0.96, 0.96, 0.96, 1.0),
            0.0,
            u32::MAX,
        )
    }

    pub fn create_shader(
        &mut self,
        name: impl Into<String>,
        position: Vec2,
        size: Vec2,
        shader_id: u32,
    ) -> NodeId {
        self.push_common(
            name.into(),
            LayerType::Shader,
            position,
            size,
            Color::BLACK,
            0.0,
            shader_id,
        )
    }

    pub fn set_position(&mut self, id: NodeId, position: Vec2) {
        let i = id.index();
        if !self.is_alive(id) {
            return;
        }
        self.transforms[i].position = position;
        self.dirty[i].insert(DirtyFlags::TRANSFORM);
        self.world_dirty = true;
        self.instance_dirty = true;
    }

    pub fn translate(&mut self, id: NodeId, delta: Vec2) {
        let i = id.index();
        if !self.is_alive(id) {
            return;
        }
        self.transforms[i].position += delta;
        self.dirty[i].insert(DirtyFlags::TRANSFORM);
        self.world_dirty = true;
        self.instance_dirty = true;
    }

    pub fn set_size(&mut self, id: NodeId, size: Vec2) {
        let i = id.index();
        if !self.is_alive(id) {
            return;
        }
        self.sizes[i] = size;
        self.dirty[i].insert(DirtyFlags::GEOMETRY);
        self.instance_dirty = true;
    }

    pub fn set_fill(&mut self, id: NodeId, fill: Color) {
        let i = id.index();
        if !self.is_alive(id) {
            return;
        }
        if self.layer_types[i] == LayerType::Shader {
            return;
        }
        self.fills[i] = fill;
        self.dirty[i].insert(DirtyFlags::MATERIAL);
        self.instance_dirty = true;
    }

    pub fn set_radius(&mut self, id: NodeId, radius: f32) {
        let i = id.index();
        if !self.is_alive(id) {
            return;
        }
        self.radii[i] = radius.max(0.0);
        self.dirty[i].insert(DirtyFlags::GEOMETRY);
        self.instance_dirty = true;
    }

    pub fn set_opacity(&mut self, id: NodeId, opacity: f32) {
        let i = id.index();
        if !self.is_alive(id) {
            return;
        }
        self.opacities[i] = opacity.clamp(0.0, 1.0);
        self.dirty[i].insert(DirtyFlags::MATERIAL);
        self.instance_dirty = true;
    }

    pub fn set_visibility(&mut self, id: NodeId, visible: bool) {
        let i = id.index();
        if !self.is_alive(id) {
            return;
        }
        self.visibility[i] = visible;
        self.dirty[i].insert(DirtyFlags::VISIBILITY);
        self.instance_dirty = true;
    }

    pub fn set_locked(&mut self, id: NodeId, locked: bool) {
        let i = id.index();
        if !self.is_alive(id) {
            return;
        }
        self.locked[i] = locked;
    }

    pub fn set_name(&mut self, id: NodeId, name: String) {
        let i = id.index();
        if !self.is_alive(id) {
            return;
        }
        self.names[i] = name;
    }

    pub fn is_alive(&self, id: NodeId) -> bool {
        let i = id.index();
        i < self.alive.len() && self.alive[i]
    }

    pub fn is_locked(&self, id: NodeId) -> bool {
        self.is_alive(id) && self.locked[id.index()]
    }

    pub fn has_visible_shaders(&self) -> bool {
        for id in self.paint_order() {
            if self.layer_types[id.index()] == LayerType::Shader {
                return true;
            }
        }
        false
    }

    pub fn local_bounds(&self, id: NodeId) -> Option<Rect> {
        if !self.is_alive(id) {
            return None;
        }
        let i = id.index();
        let pos = self.transforms[i].position;
        let size = self.sizes[i];
        Some(Rect::from_pos_size(pos, size))
    }

    pub fn world_bounds(&self, id: NodeId) -> Option<Rect> {
        self.local_bounds(id)
    }

    pub fn update_world_transforms(&mut self) {
        if !self.world_dirty {
            return;
        }
        for i in 0..self.len() {
            if !self.alive[i] {
                continue;
            }
            self.world_transforms[i] = self.transforms[i].to_matrix();
            self.dirty[i].remove(DirtyFlags::TRANSFORM);
        }
        self.world_dirty = false;
    }

    pub fn paint_order(&self) -> Vec<NodeId> {
        self.roots
            .iter()
            .copied()
            .filter(|id| self.is_alive(*id) && self.visibility[id.index()])
            .collect()
    }

    pub fn snapshot_node(&self, id: NodeId) -> Option<NodeSnapshot> {
        if !self.is_alive(id) {
            return None;
        }
        let i = id.index();
        let fill = self.fills[i];
        Some(NodeSnapshot {
            id,
            name: self.names[i].clone(),
            layer_type: self.layer_types[i],
            visible: self.visibility[i],
            locked: self.locked[i],
            x: self.transforms[i].position.x,
            y: self.transforms[i].position.y,
            width: self.sizes[i].x,
            height: self.sizes[i].y,
            fill: fill.to_array(),
            radius: self.radii[i],
            opacity: self.opacities[i],
            shader_id: self.shader_ids[i],
        })
    }

    pub fn layer_list(&self) -> Vec<NodeSnapshot> {
        self.roots
            .iter()
            .rev()
            .filter_map(|id| self.snapshot_node(*id))
            .collect()
    }
}
