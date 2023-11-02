use std::fmt::{Display, Debug};

use crate::geom::Vector2d;

/// Represents a text.
#[derive(Clone, PartialEq)]
pub struct Text {
    pub position: Vector2d,
    pub text: String,
    pub font_size: f64,
    /// CSS font family. This field supports specifying
    /// fallback fonts by using the comma delimiter.
    pub font_family: String,

    pub font_weight: u32,
    /// Indicates a lightweight font. If true, overrides the
    /// `font_weight` field.
    pub light: bool,
    /// Indicates a bold font. If true, overrides the
    /// `font_weight` field.
    pub bold: bool,

    pub italic: bool,

    pub strikethrough: bool,
    pub underline: bool,
}

impl Default for Text {
    fn default() -> Self {
        Self {
            position: Vector2d(0.0, 0.0),
            text: "".into(),
            font_size: 16.0,
            font_family: "".into(),
            font_weight: 400,
            light: false,
            bold: false,
            italic: false,
            strikethrough: false,
            underline: false,
        }
    }
}

impl Debug for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

impl Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Text(position={}, ...)", self.position)
    }
}

impl Text {
    pub fn position(&self) -> Vector2d {
        self.position
    }

    pub fn set_position(&mut self, value: &Vector2d) {
        self.position = value.clone();
    }

    pub fn text(&self) -> String {
        self.text.clone()
    }

    pub fn set_text(&mut self, value: String) {
        self.text = value;
    }

    pub fn font_size(&self) -> f64 {
        self.font_size.clone()
    }

    pub fn set_font_size(&mut self, value: f64) {
        self.font_size = value;
    }

    pub fn font_family(&self) -> String {
        self.font_family.clone()
    }

    pub fn set_font_family(&mut self, value: String) {
        self.font_family = value;
    }
}