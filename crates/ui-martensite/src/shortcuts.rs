//! Keystroke state machine providing Illustrator keyboard ergonomics.

use vectorcraft_engine::ToolId;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KeyModifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub cmd: bool,
}

impl KeyModifiers {
    pub const fn empty() -> Self {
        Self { shift: false, ctrl: false, alt: false, cmd: false }
    }
}

pub struct KeyboardEngine {
    pub prior_tool: Option<ToolId>,
    pub space_held: bool,
    pub z_held: bool,
    pub alt_held: bool,
}

impl KeyboardEngine {
    pub fn new() -> Self {
        Self { prior_tool: None, space_held: false, z_held: false, alt_held: false }
    }

    pub fn on_key_down(&mut self, key: &str, current: ToolId) -> Option<ToolId> {
        match key {
            "Space" if !self.space_held => {
                self.space_held = true;
                self.prior_tool = Some(current);
                Some(ToolId::Hand)
            }
            "z" | "Z" if !self.z_held => {
                self.z_held = true;
                self.prior_tool = Some(current);
                Some(ToolId::Zoom)
            }
            "Alt" => {
                self.alt_held = true;
                None
            }
            // Standard Illustrator single-key tool shortcuts
            "v" | "V" => Some(ToolId::Selection),
            "a" | "A" => Some(ToolId::DirectSelection),
            "p" | "P" => Some(ToolId::Pen),
            "t" | "T" => Some(ToolId::Type),
            "m" | "M" => Some(ToolId::Rectangle),
            "l" | "L" => Some(ToolId::Ellipse),
            "r" | "R" => Some(ToolId::Rotate),
            "s" | "S" => Some(ToolId::Scale),
            "h" | "H" => Some(ToolId::Hand),
            // Shift-modified tool keys
            "Shift+C" => Some(ToolId::AnchorConvert),
            "Shift+M" => Some(ToolId::ShapeBuilder),
            _ => None,
        }
    }

    pub fn on_key_up(&mut self, key: &str) -> Option<ToolId> {
        match key {
            "Space" if self.space_held => {
                self.space_held = false;
                self.prior_tool.take()
            }
            "z" | "Z" if self.z_held => {
                self.z_held = false;
                self.prior_tool.take()
            }
            "Alt" => {
                self.alt_held = false;
                None
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_key_tool_switching() {
        let mut k = KeyboardEngine::new();
        assert_eq!(k.on_key_down("v", ToolId::Pen), Some(ToolId::Selection));
        assert_eq!(k.on_key_down("A", ToolId::Selection), Some(ToolId::DirectSelection));
        assert_eq!(k.on_key_down("p", ToolId::DirectSelection), Some(ToolId::Pen));
        assert_eq!(k.on_key_down("t", ToolId::Pen), Some(ToolId::Type));
        assert_eq!(k.on_key_down("m", ToolId::Type), Some(ToolId::Rectangle));
        assert_eq!(k.on_key_down("l", ToolId::Rectangle), Some(ToolId::Ellipse));
        assert_eq!(k.on_key_down("r", ToolId::Ellipse), Some(ToolId::Rotate));
        assert_eq!(k.on_key_down("s", ToolId::Rotate), Some(ToolId::Scale));
        assert_eq!(k.on_key_down("h", ToolId::Scale), Some(ToolId::Hand));
        assert_eq!(k.on_key_down("Shift+C", ToolId::Pen), Some(ToolId::AnchorConvert));
        assert_eq!(k.on_key_down("Shift+M", ToolId::Selection), Some(ToolId::ShapeBuilder));
    }

    #[test]
    fn test_spring_loaded_hand_tool() {
        let mut k = KeyboardEngine::new();
        let initial = ToolId::Pen;

        // Press Space: temporary Hand
        assert_eq!(k.on_key_down("Space", initial), Some(ToolId::Hand));
        assert!(k.space_held);

        // Multiple down events shouldn't overwrite prior tool
        assert_eq!(k.on_key_down("Space", ToolId::Hand), None);

        // Release Space: restores initial tool
        assert_eq!(k.on_key_up("Space"), Some(initial));
        assert!(!k.space_held);
    }

    #[test]
    fn test_spring_loaded_zoom_tool() {
        let mut k = KeyboardEngine::new();
        let initial = ToolId::DirectSelection;

        assert_eq!(k.on_key_down("z", initial), Some(ToolId::Zoom));
        assert!(k.z_held);

        assert_eq!(k.on_key_up("z"), Some(initial));
        assert!(!k.z_held);
    }
}
