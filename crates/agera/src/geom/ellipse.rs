use std::fmt::{Display, Debug};

use crate::geom::Vector2d;

/// Represents an ellipse.
#[derive(Copy, Clone, PartialEq)]
pub struct Ellipse {
    pub position: Vector2d,
    pub size: Vector2d,
}

impl Debug for Ellipse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

impl Display for Ellipse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(position={}, size={})", self.position, self.size)
    }
}

impl Ellipse {
    pub fn position(&self) -> Vector2d {
        self.position
    }

    pub fn set_position(&mut self, value: &Vector2d) {
        self.position = value.clone();
    }

    pub fn size(&self) -> Vector2d {
        self.size
    }

    pub fn set_size(&mut self, value: &Vector2d) {
        self.size = value.clone();
    }
}