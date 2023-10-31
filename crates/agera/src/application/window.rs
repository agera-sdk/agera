use crate::entity::*;

/// Represents a window. For browser applications, there can only be
/// a single `Window` object.
pub struct Window {
    pub(crate) root: Entity,
}

impl Window {
    /// Returns the root entity of the window.
    pub fn root(&self) -> Entity {
        self.root.clone()
    }
}