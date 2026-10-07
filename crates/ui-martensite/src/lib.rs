//! Sovereign retained-mode vector UI for VectorCraft built on Martensite.

pub mod command_reg;
pub mod menus;
pub mod pathfinder;
pub mod shortcuts;
pub mod theme;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VectorTool {
    Selection,       // V
    DirectSelection, // A
    Pen,             // P
    AnchorConvert,   // Shift+C
    Rectangle,       // M
    Ellipse,         // L
    ShapeBuilder,    // Shift+M
    Rotate,          // R
}

pub struct VectorcraftApp {
    pub active_tool: VectorTool,
    pub stroke_width: f32,
    pub fill_enabled: bool,
    pub stroke_enabled: bool,
}

impl VectorcraftApp {
    pub fn new() -> Self {
        Self {
            active_tool: VectorTool::Selection,
            stroke_width: 1.0,
            fill_enabled: true,
            stroke_enabled: true,
        }
    }

    pub fn set_stroke_width(&mut self, w: f32) {
        self.stroke_width = w.clamp(0.0, 1000.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state() {
        let mut app = VectorcraftApp::new();
        assert_eq!(app.active_tool, VectorTool::Selection);
        app.set_stroke_width(5.5);
        assert_eq!(app.stroke_width, 5.5);
    }
}
