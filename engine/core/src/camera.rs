use crate::{Mat3, Vec2};
use serde::{Deserialize, Serialize};

/// Orthographic camera in world space.
///
/// Screen space: origin top-left, Y down (browser).
/// World space: Y up, origin at camera center by default.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Camera {
    /// World-space position of the view center.
    pub position: Vec2,
    /// World units per screen pixel inverse: larger = zoomed in.
    pub zoom: f32,
    pub min_zoom: f32,
    pub max_zoom: f32,
    /// Viewport size in CSS pixels.
    pub viewport_size: Vec2,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            zoom: 1.0,
            min_zoom: 0.02,
            max_zoom: 64.0,
            viewport_size: Vec2::new(800.0, 600.0),
        }
    }
}

impl Camera {
    pub fn set_viewport(&mut self, width: f32, height: f32) {
        self.viewport_size = Vec2::new(width.max(1.0), height.max(1.0));
    }

    pub fn pan(&mut self, delta_screen: Vec2) {
        self.position -= delta_screen / self.zoom;
    }

    /// Zoom centered on a screen-space cursor so the world point under the cursor stays fixed.
    pub fn zoom_at(&mut self, cursor_screen: Vec2, factor: f32) {
        let world_before = self.screen_to_world(cursor_screen);
        let new_zoom = (self.zoom * factor).clamp(self.min_zoom, self.max_zoom);
        self.zoom = new_zoom;
        // newCameraPosition = worldPoint - cursorPosition / newZoom
        // With Y-down screen coords mapped into world:
        let half = self.viewport_size * 0.5;
        let offset = (cursor_screen - half) / self.zoom;
        // screen_to_world: world = position + (screen - half) / zoom * (1, -1)
        // Keep world_before under cursor:
        self.position = Vec2::new(world_before.x - offset.x, world_before.y + offset.y);
    }

    pub fn screen_to_world(&self, screen: Vec2) -> Vec2 {
        let half = self.viewport_size * 0.5;
        let local = (screen - half) / self.zoom;
        Vec2::new(self.position.x + local.x, self.position.y - local.y)
    }

    pub fn world_to_screen(&self, world: Vec2) -> Vec2 {
        let half = self.viewport_size * 0.5;
        let local = Vec2::new(world.x - self.position.x, self.position.y - world.y);
        half + local * self.zoom
    }

    /// View-projection matrix for WGSL (clip space, Y up).
    /// Maps world → NDC.
    pub fn view_proj_matrix(&self) -> Mat3 {
        let w = self.viewport_size.x.max(1.0);
        let h = self.viewport_size.y.max(1.0);
        // Scale world so that viewport width maps to NDC [-1, 1]
        let sx = (2.0 * self.zoom) / w;
        let sy = (2.0 * self.zoom) / h;
        let tx = -self.position.x * sx;
        let ty = -self.position.y * sy;
        Mat3::from_cols(
            glam::Vec3::new(sx, 0.0, 0.0),
            glam::Vec3::new(0.0, sy, 0.0),
            glam::Vec3::new(tx, ty, 1.0),
        )
    }

    pub fn visible_world_bounds(&self) -> crate::Rect {
        let half = self.viewport_size * 0.5 / self.zoom;
        crate::Rect {
            min: self.position - half,
            max: self.position + half,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zoom_keeps_world_point_stable() {
        let mut cam = Camera::default();
        cam.set_viewport(1000.0, 800.0);
        cam.position = Vec2::new(100.0, 50.0);
        cam.zoom = 1.0;
        let cursor = Vec2::new(400.0, 300.0);
        let world = cam.screen_to_world(cursor);
        cam.zoom_at(cursor, 2.0);
        let world_after = cam.screen_to_world(cursor);
        assert!((world - world_after).length() < 0.01);
    }

    #[test]
    fn screen_world_roundtrip() {
        let mut cam = Camera::default();
        cam.set_viewport(800.0, 600.0);
        cam.position = Vec2::new(10.0, -20.0);
        cam.zoom = 1.5;
        let screen = Vec2::new(123.0, 456.0);
        let world = cam.screen_to_world(screen);
        let back = cam.world_to_screen(world);
        assert!((screen - back).length() < 0.01);
    }
}
