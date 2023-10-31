use crate::display::*;

/// Represents a window. For browser applications, there can only be
/// a single `Window` object.
pub struct Window {
    pub(crate) root: DisplayObject,
}

impl Window {
    /// Returns the root display object of the window.
    pub fn root(&self) -> DisplayObject {
        self.root.clone()
    }
}