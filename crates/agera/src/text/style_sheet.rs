use crate::{common::*, util::Color};

/// Text formatting rules for font size, color, and other styles.
#[derive(Clone)]
pub struct StyleSheet {
    /// The host style. This is equivalent to the CSS `:host` selector.
    pub host: StyleSheetStyle,
    /// The anchor style. This is equivalent to the CSS `a` selector.
    pub anchor: StyleSheetStyle,
    /// The anchor style on hover. This is equivalent to the CSS `a:hover` selector.
    pub anchor_hover: StyleSheetStyle,
    /// Selection style. This is equivalent to the CSS `::selection` selector.
    pub selection: StyleSheetStyle,
}

impl Default for StyleSheet {
    fn default() -> Self {
        Self {
            host: default(),
            anchor: StyleSheetStyle {
                ..default()
            },
            anchor_hover: StyleSheetStyle {
                text_decoration: Some(StyleSheetTextDecoration::Underline),
                ..default()
            },
            selection: StyleSheetStyle {
                background_color: Some(Color::new(0.0, 0.0, 0.0, 1.0)),
                color: Some(Color::new(1.0, 1.0, 1.0, 1.0)),
                ..default()
            },
        }
    }
}

/// Style applicable to text fields.
#[derive(Clone)]
pub struct StyleSheetStyle {
    pub font_size: Option<StyleSheetUnits>,
    /// Font family. This field supports specifying fallback fonts by
    /// using the comma delimiter.
    /// This is equivalent to the CSS property `font-family`.
    pub font_family: Option<String>,

    pub font_weight: Option<u32>,
    /// Indicates a lightweight font. If set, overrides the `font_weight` field.
    /// This is equivalent to the CSS `font-weight: lighter;` syntax.
    pub lighter: Option<bool>,
    /// Indicates a bold font. If set, overrides the `font_weight` field.
    /// This is equivalent to the CSS `font-weight: bold;` syntax.
    pub bold: Option<bool>,

    /// Indicates an italic font.
    /// This is equivalent to the CSS `font-style: italic;` syntax.
    pub italic: Option<bool>,

    /// Text color. This color applies to text and symbols such as list bullets.
    pub color: Option<Color>,

    /// Background color. This is equivalent to the CSS `background: <color>;`
    /// and `background-color: <color>;` syntaxes.
    pub background_color: Option<Color>,

    pub text_decoration: Option<StyleSheetTextDecoration>,

    pub text_transform: Option<StyleSheetTextTransform>,
}

impl Default for StyleSheetStyle {
    fn default() -> Self {
        Self {
            font_size: None,
            font_family: None,
            font_weight: None,
            lighter: None,
            bold: None,
            italic: None,
            color: None,
            background_color: None,
            text_decoration: None,
            text_transform: None,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum StyleSheetTextDecoration {
    None,
    Underline,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum StyleSheetTextTransform {
    None,
    Capitalize,
    Lowercase,
    Uppercase,
}

#[derive(Copy, Clone)]
pub enum StyleSheetUnits {
    Pixels(f64),
}