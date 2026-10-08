//! Tool strip widget: single/double column layout, tool flyouts, and fill/stroke chips.

use vectorcraft_engine::ToolId;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ToolSlot {
    pub primary: ToolId,
    pub alternatives: &'static [ToolId],
}

/// The toolbox in Illustrator order; flyout tools ride under their group's primary.
pub const TOOL_SLOTS: &[ToolSlot] = &[
    ToolSlot { primary: ToolId::Selection, alternatives: &[ToolId::DirectSelection] },
    ToolSlot { primary: ToolId::Pen, alternatives: &[ToolId::AnchorConvert] },
    ToolSlot { primary: ToolId::Type, alternatives: &[] },
    ToolSlot { primary: ToolId::Rectangle, alternatives: &[ToolId::Ellipse] },
    ToolSlot { primary: ToolId::ShapeBuilder, alternatives: &[] },
    ToolSlot { primary: ToolId::Rotate, alternatives: &[ToolId::Scale] },
    ToolSlot { primary: ToolId::Hand, alternatives: &[] },
    ToolSlot { primary: ToolId::Zoom, alternatives: &[] },
];

pub struct ToolStripWidget {
    pub active_tool: ToolId,
    pub double_column: bool,
    pub fill_color: [u8; 4],
    pub stroke_color: [u8; 4],
    /// Which chip sits in front of the other (the X key): true = Fill, false = Stroke.
    pub fill_active: bool,
}

impl ToolStripWidget {
    pub fn new() -> Self {
        Self {
            active_tool: ToolId::Selection,
            double_column: false,
            fill_color: [255, 255, 255, 255], // Default white fill
            stroke_color: [0, 0, 0, 255],     // Default black stroke
            fill_active: true,
        }
    }

    pub fn toggle_column_mode(&mut self) -> bool {
        self.double_column = !self.double_column;
        self.double_column
    }

    /// X: bring the other chip (fill or stroke) to the front.
    pub fn toggle_fill_active(&mut self) -> bool {
        self.fill_active = !self.fill_active;
        self.fill_active
    }

    /// Shift+X: swap the fill and stroke colors.
    pub fn swap_fill_stroke(&mut self) {
        std::mem::swap(&mut self.fill_color, &mut self.stroke_color);
    }

    /// D: white fill, black stroke.
    pub fn reset_default_colors(&mut self) {
        self.fill_color = [255, 255, 255, 255];
        self.stroke_color = [0, 0, 0, 255];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_strip_state() {
        let mut strip = ToolStripWidget::new();
        assert_eq!(strip.active_tool, ToolId::Selection);
        assert!(!strip.double_column);
        assert_eq!(strip.fill_color, [255, 255, 255, 255]);
        assert_eq!(strip.stroke_color, [0, 0, 0, 255]);
        assert!(strip.fill_active);

        strip.swap_fill_stroke();
        assert_eq!(strip.fill_color, [0, 0, 0, 255]);
        assert_eq!(strip.stroke_color, [255, 255, 255, 255]);

        strip.reset_default_colors();
        assert_eq!(strip.fill_color, [255, 255, 255, 255]);
        assert_eq!(strip.stroke_color, [0, 0, 0, 255]);

        assert!(!strip.toggle_fill_active());
        assert!(strip.toggle_fill_active());

        assert!(strip.toggle_column_mode());
        assert!(strip.double_column);
        assert!(!strip.toggle_column_mode());
    }

    #[test]
    fn test_tool_slots_cover_toolbox() {
        assert_eq!(TOOL_SLOTS.len(), 8);
        assert_eq!(TOOL_SLOTS[0].primary, ToolId::Selection);
        assert!(TOOL_SLOTS[0].alternatives.contains(&ToolId::DirectSelection));
        assert!(TOOL_SLOTS[1].alternatives.contains(&ToolId::AnchorConvert));
    }
}
