use crate::{common::*, display::*, entity::*, geom::*, text::*};

entity_type! {
    use agera = crate;
    /// Represents a text area of fixed size.
    pub struct TextField: DisplayObject < Entity {
        pub size: Vector2d = Vector2d(100.0, 50.0),
        pub text: String = "".into(),
        /// The default text formatting options used by this text area.
        pub text_format: TextFormat = default(),
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