use crate::util::Color;

/// Text formatting options.
#[derive(Clone, PartialEq)]
pub struct TextFormat {
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

    pub superscript: bool,
    pub subscript: bool,

    /// Text color. This color applies to text and symbols such as list bullets.
    pub color: Color,
    /// Anchor color. This color applies to anchor links.
    pub anchor_color: Color,
    /// Selected text color.
    pub selected_color: Color,

    pub selected_background_color: Color,
}

impl Default for TextFormat {
    fn default() -> Self {
        Self {
            font_size: 16.0,
            font_family: "".into(),
            font_weight: 400,
            light: false,
            bold: false,
            italic: false,
            strikethrough: false,
            underline: false,
            superscript: false,
            subscript: false,
            color: Color::new(1.0, 1.0, 1.0, 1.0),
            anchor_color: Color::new(0.0, 0.0, 0.0, 1.0),
            selected_color: Color::new(1.0, 1.0, 1.0, 1.0),
            selected_background_color: Color::new(0.0, 0.0, 0.0, 1.0),
        }
    }
}