use crate::{common::*, display::*, entity::*, geom::*, util::Color};

entity_type! {
    use agera = crate;
    /// Represents a text field of fixed size.
    /// 
    /// # HTML support
    /// 
    /// `TextField` supports assigning a limited dialect of HTML text
    /// through its `set_html()` method.
    /// It supports HTML entities and a subset of the HTML elements.
    /// 
    /// | Tag                | Description                                     |
    /// | ------------------ | ----------------------------------------------- |
    /// | `<b></b>`          | Bold |
    /// | `<i></i>`          | Italic |
    /// | `<s></s>`          | Strikethrough |
    /// | `<u></u>`          | Underline text |
    /// | `<sup></sup>`      | Superscript |
    /// | `<sub></sub>`      | Subscript |
    /// | `<a></a>`          | Anchor, supporting the `href` attribute |
    /// | `<ul></ul>`        | Unordered list |
    /// | `<ol></ol>`        | Ordered list |
    /// | `<li></li>`        | List item |
    /// | `<p></p>`          | Paragraph |
    /// | `<center></center>` | Centered content |
    /// 
    pub struct TextField: DisplayObject < Entity {
        pub size: Vector2d = Vector2d(100.0, 50.0),
        pub style_sheet: TextFieldCss = default(),
        pub horizontal_alignment: TextHorizontalAlignment = TextHorizontalAlignment::Left,
        pub vertical_alignment: TextVerticalAlignment = TextVerticalAlignment::Top,
        /// Horizontal scroll, in pixels.
        pub horizontal_scroll: f64 = 0.0,
        /// Vertical scroll, in pixels.
        pub vertical_scroll: f64 = 0.0,
    }

    pub fn constructor(text: String) {
        super();
        this.set_text(text.clone());
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TextHorizontalAlignment {
    Left,
    Center,
    Right,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TextVerticalAlignment {
    Top,
    Center,
    Bottom,
}

/// Style sheet applicable to text fields.
#[derive(Clone, PartialEq)]
pub struct TextFieldCss {
    /// The host style. This is similiar to the CSS `:host` selector.
    pub host: TextFieldCssStyle,
    /// The anchor style. This is similiar to the CSS `a` selector.
    pub anchor: TextFieldCssStyle,
    /// The anchor style on hover. This is similiar to the CSS `a:hover` selector.
    pub anchor_hover: TextFieldCssStyle,
    /// Selection style. This is similiar to the CSS `::selection` selector.
    pub selection: TextFieldCssStyle,
}

impl Default for TextFieldCss {
    fn default() -> Self {
        Self {
            host: default(),
            anchor: TextFieldCssStyle {
                ..default()
            },
            anchor_hover: TextFieldCssStyle {
                text_decoration: Some(TextFieldCssTextDecoration::Underline),
                ..default()
            },
            selection: TextFieldCssStyle {
                background_color: Some(Color::new(0.0, 0.0, 0.0, 1.0)),
                color: Some(Color::new(1.0, 1.0, 1.0, 1.0)),
                ..default()
            },
        }
    }
}

/// Style applicable to text fields.
#[derive(Clone, PartialEq)]
pub struct TextFieldCssStyle {
    pub font_size: Option<f64>,
    /// Font family. This field supports specifying fallback fonts by
    /// using the comma delimiter.
    /// This is similiar to the CSS property `font-family`.
    pub font_family: Option<String>,

    pub font_weight: Option<u32>,
    /// Indicates a lightweight font. If set, overrides the `font_weight` field.
    /// This is similiar to the CSS `font-weight: lighter;` syntax.
    pub lighter: Option<bool>,
    /// Indicates a bold font. If set, overrides the `font_weight` field.
    /// This is similiar to the CSS `font-weight: bold;` syntax.
    pub bold: Option<bool>,

    /// Indicates an italic font.
    /// This is similiar to the CSS `font-style: italic;` syntax.
    pub italic: Option<bool>,

    /// Text color. This color applies to text and symbols such as list bullets.
    pub color: Option<Color>,

    /// Background color. This is similiar to the CSS `background: <color>;`
    /// and `background-color: <color>;` syntaxes.
    pub background_color: Option<Color>,

    pub text_decoration: Option<TextFieldCssTextDecoration>,
}

impl Default for TextFieldCssStyle {
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
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TextFieldCssTextDecoration {
    None,
    Underline,
}