//! Vector workspace design tokens.

pub struct Color(pub u8, pub u8, pub u8);

pub struct Theme {
    pub canvas_bg: Color,
    pub anchor_handle: Color,
}

impl Theme {
    pub fn vector_studio() -> Self {
        Self {
            canvas_bg: Color(28, 30, 36),
            anchor_handle: Color(0, 215, 255),
        }
    }
}
