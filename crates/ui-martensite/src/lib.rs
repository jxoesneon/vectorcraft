//! Sovereign retained-mode interface for VectorCraft built on the Martensite GUI engine.

pub mod command_reg;
pub mod menus;
pub mod pathfinder;
pub mod shortcuts;
pub mod theme;
pub mod widgets;

use std::sync::{Arc, Mutex};

use vectorcraft_engine::Engine;
pub use vectorcraft_engine::ToolId;

/// Application state container managing the Martensite GUI pipeline.
pub struct VectorcraftApp {
    pub engine: Arc<Mutex<Engine>>,
    pub theme: theme::CraftTheme,
    pub keyboard: shortcuts::KeyboardEngine,
    pub active_tool: ToolId,
    pub zoom_level: f32,
    pub pan_offset: [f32; 2],
    pub rulers_visible: bool,
    pub outline_preview: bool,
    pub stroke_width: f32,
    pub fill_enabled: bool,
    pub stroke_enabled: bool,
    pub is_dirty: bool,
}

impl VectorcraftApp {
    pub fn new(engine: Engine) -> Self {
        Self {
            engine: Arc::new(Mutex::new(engine)),
            theme: theme::CraftTheme::dark_neutral(),
            keyboard: shortcuts::KeyboardEngine::new(),
            active_tool: ToolId::Selection,
            zoom_level: 1.0,
            pan_offset: [0.0, 0.0],
            rulers_visible: true,
            outline_preview: false,
            stroke_width: 1.0,
            fill_enabled: true,
            stroke_enabled: true,
            is_dirty: false,
        }
    }

    pub fn set_tool(&mut self, tool: ToolId) {
        self.active_tool = tool;
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom_level = zoom.clamp(0.01, 64.0);
    }

    pub fn pan_by(&mut self, dx: f32, dy: f32) {
        self.pan_offset[0] += dx;
        self.pan_offset[1] += dy;
    }

    pub fn reset_view(&mut self) {
        self.zoom_level = 1.0;
        self.pan_offset = [0.0, 0.0];
    }

    pub fn toggle_rulers(&mut self) -> bool {
        self.rulers_visible = !self.rulers_visible;
        self.rulers_visible
    }

    /// Cmd+Y: wireframe preview of the artwork's geometry (View › Outline).
    pub fn toggle_outline_preview(&mut self) -> bool {
        self.outline_preview = !self.outline_preview;
        self.outline_preview
    }

    pub fn set_stroke_width(&mut self, w: f32) {
        self.stroke_width = w.clamp(0.0, 1000.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_initialization() {
        let engine = Engine::new();
        let app = VectorcraftApp::new(engine);
        assert_eq!(app.active_tool, ToolId::Selection);
        assert_eq!(app.zoom_level, 1.0);
        assert_eq!(app.pan_offset, [0.0, 0.0]);
        assert!(app.rulers_visible);
        assert!(!app.outline_preview);
        assert!(app.fill_enabled);
        assert!(app.stroke_enabled);
        assert!(!app.is_dirty);
    }

    #[test]
    fn test_zoom_clamping() {
        let engine = Engine::new();
        let mut app = VectorcraftApp::new(engine);

        app.set_zoom(2.5);
        assert_eq!(app.zoom_level, 2.5);

        app.set_zoom(0.0001);
        assert_eq!(app.zoom_level, 0.01);

        app.set_zoom(1000.0);
        assert_eq!(app.zoom_level, 64.0);
    }

    #[test]
    fn test_pan_and_reset() {
        let engine = Engine::new();
        let mut app = VectorcraftApp::new(engine);

        app.pan_by(120.0, -45.0);
        assert_eq!(app.pan_offset, [120.0, -45.0]);

        app.set_zoom(3.0);
        app.reset_view();
        assert_eq!(app.zoom_level, 1.0);
        assert_eq!(app.pan_offset, [0.0, 0.0]);
    }

    #[test]
    fn test_toggles_and_stroke_width() {
        let engine = Engine::new();
        let mut app = VectorcraftApp::new(engine);

        assert!(app.rulers_visible);
        assert!(!app.toggle_rulers());
        assert!(!app.rulers_visible);
        assert!(app.toggle_rulers());

        assert!(!app.outline_preview);
        assert!(app.toggle_outline_preview());
        assert!(app.outline_preview);
        assert!(!app.toggle_outline_preview());

        app.set_stroke_width(5.5);
        assert_eq!(app.stroke_width, 5.5);
        app.set_stroke_width(-2.0);
        assert_eq!(app.stroke_width, 0.0);
        app.set_stroke_width(5000.0);
        assert_eq!(app.stroke_width, 1000.0);
    }
}
