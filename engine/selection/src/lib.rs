//! Hit testing and selection state.

use engine_core::{NodeId, Vec2};
use engine_scene::Scene;

#[derive(Debug, Default, Clone)]
pub struct SelectionState {
    pub selected: Vec<NodeId>,
}

impl SelectionState {
    pub fn clear(&mut self) {
        self.selected.clear();
    }

    pub fn primary(&self) -> Option<NodeId> {
        self.selected.first().copied()
    }

    pub fn contains(&self, id: NodeId) -> bool {
        self.selected.contains(&id)
    }

    pub fn set(&mut self, id: NodeId) {
        self.selected.clear();
        self.selected.push(id);
    }

    pub fn toggle(&mut self, id: NodeId) {
        if let Some(pos) = self.selected.iter().position(|x| *x == id) {
            self.selected.remove(pos);
        } else {
            self.selected.push(id);
        }
    }

    pub fn set_many(&mut self, ids: Vec<NodeId>) {
        self.selected = ids;
    }
}

/// Top-most hit under a world-space point (reverse paint order).
pub fn hit_test(scene: &Scene, world: Vec2) -> Option<NodeId> {
    let order = scene.paint_order();
    for id in order.into_iter().rev() {
        if scene.is_locked(id) {
            continue;
        }
        if let Some(bounds) = scene.world_bounds(id) {
            if bounds.contains_point(world) {
                return Some(id);
            }
        }
    }
    None
}

/// Nodes whose bounds intersect the world-space rect.
pub fn rect_select(scene: &Scene, rect: engine_core::Rect) -> Vec<NodeId> {
    let mut result = Vec::new();
    for id in scene.paint_order() {
        if scene.is_locked(id) {
            continue;
        }
        if let Some(bounds) = scene.world_bounds(id) {
            if bounds.intersects(rect) {
                result.push(id);
            }
        }
    }
    result
}
