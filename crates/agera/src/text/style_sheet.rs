use crate::{common::*, util::Color};

/// Text formatting rules for font size, color, and other styles.
#[derive(Clone)]
pub struct StyleSheetContainer {
    /// The host style. This is equivalent to the CSS `:host` selector.
    pub host: StyleSheet,
    /// The anchor style. This is equivalent to the CSS `a` selector.
    pub anchor: StyleSheet,
    /// The anchor style on hover. This is equivalent to the CSS `a:hover` selector.
    pub anchor_hover: StyleSheet,
    /// Selection style. This is equivalent to the CSS `::selection` selector.
    pub selection: StyleSheet,
    /// Heading title styles.
    pub heading_title: HashMap<usize, StyleSheet>,
}

impl Default for StyleSheetContainer {
    fn default() -> Self {
        Self {
            host: default(),
            anchor: default(),
            anchor_hover: with! {
                text_decoration: Some(TextDecoration::Underline),
                ..
            },
            selection: with! {
                background_color: Some(Color::new(0.0, 0.0, 0.0, 1.0)),
                color: Some(Color::new(1.0, 1.0, 1.0, 1.0)),
                ..
            },
            heading_title: hashmap! {},
        }
    }
}

/// Style applicable to text fields.
#[derive(Clone)]
pub struct StyleSheet {
    /// Font size, in points.
    pub font_size: Option<f64>,
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

    pub text_decoration: Option<TextDecoration>,

    pub text_transform: Option<TextTransform>,
}

impl Default for StyleSheet {
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
pub enum TextDecoration {
    None,
    Underline,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TextTransform {
    None,
    Capitalize,
    Lowercase,
    Uppercase,
}