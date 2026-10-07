//! Vector tool shortcuts.

use crate::VectorTool;

pub fn map_shortcut(key: &str) -> Option<VectorTool> {
    match key {
        "v" | "V" => Some(VectorTool::Selection),
        "a" | "A" => Some(VectorTool::DirectSelection),
        "p" | "P" => Some(VectorTool::Pen),
        "m" | "M" => Some(VectorTool::Rectangle),
        "l" | "L" => Some(VectorTool::Ellipse),
        "r" | "R" => Some(VectorTool::Rotate),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_shortcuts() {
        assert_eq!(map_shortcut("p"), Some(VectorTool::Pen));
        assert_eq!(map_shortcut("a"), Some(VectorTool::DirectSelection));
        assert_eq!(map_shortcut("v"), Some(VectorTool::Selection));
    }
}
