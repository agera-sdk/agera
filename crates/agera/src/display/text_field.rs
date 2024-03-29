use crate::{common::*, display::*, geom::*, text::*, util::inheritance::*};

class! {
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
    /// | `<img>`            | Image, with support for `http:`, `https:`, `data:` and `file:` URLs |
    /// | `<sup></sup>`      | Superscript |
    /// | `<sub></sub>`      | Subscript |
    /// | `<a></a>`          | Anchor, supporting the `href` attribute |
    /// | `<ul></ul>`        | Unordered list |
    /// | `<ol></ol>`        | Ordered list |
    /// | `<li></li>`        | List item |
    /// | `<p></p>`          | Paragraph |
    /// | `<center></center>` | Centered content |
    /// | `<hN></hN>`         | Heading title level *N*, where *N* is one based  |
    /// | `<hr>`              | Horizontal ruler |
    /// | `<br>`              | Break |
    /// 
    pub struct TextField: DisplayObject < Node {
        pub size: Vector2d = Vector2d(100.0, 50.0),
        pub ref style_sheet: StyleSheetContainer = default(),
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