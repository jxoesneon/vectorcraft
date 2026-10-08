//! Options bar widget (the control strip) adapting dynamically to the active tool.

use vectorcraft_engine::ToolId;

use crate::widgets::scrubby_input::ScrubbyInputWidget;

pub struct OptionsBarWidget {
    pub active_tool: ToolId,
    pub stroke_weight: ScrubbyInputWidget,
    pub corner_radius: ScrubbyInputWidget,
    pub opacity: ScrubbyInputWidget,
    /// Transform › Scale Strokes & Effects for the selection.
    pub scale_strokes: bool,
    /// Snap to Pixel Grid for new art.
    pub snap_to_pixel: bool,
}

impl OptionsBarWidget {
    pub fn new() -> Self {
        Self {
            active_tool: ToolId::Selection,
            stroke_weight: ScrubbyInputWidget::new("Stroke", 1.0, 0.0, 1000.0, "pt"),
            corner_radius: ScrubbyInputWidget::new("Corners", 0.0, 0.0, 500.0, "px"),
            opacity: ScrubbyInputWidget::new("Opacity", 100.0, 0.0, 100.0, "%"),
            scale_strokes: false,
            snap_to_pixel: false,
        }
    }

    pub fn set_tool(&mut self, tool: ToolId) {
        self.active_tool = tool;
    }

    pub fn toggle_scale_strokes(&mut self) -> bool {
        self.scale_strokes = !self.scale_strokes;
        self.scale_strokes
    }

    pub fn toggle_snap_to_pixel(&mut self) -> bool {
        self.snap_to_pixel = !self.snap_to_pixel;
        self.snap_to_pixel
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_options_bar_defaults() {
        let mut bar = OptionsBarWidget::new();
        assert_eq!(bar.active_tool, ToolId::Selection);
        assert_eq!(bar.stroke_weight.value, 1.0);
        assert_eq!(bar.opacity.value, 100.0);
        assert!(!bar.scale_strokes);

        bar.set_tool(ToolId::Rectangle);
        assert_eq!(bar.active_tool, ToolId::Rectangle);

        assert!(bar.toggle_scale_strokes());
        assert!(bar.scale_strokes);
        assert!(!bar.toggle_scale_strokes());

        assert!(bar.toggle_snap_to_pixel());
        assert!(!bar.toggle_snap_to_pixel());
    }
}
