use std::fmt::{Display, Debug};

use crate::geom::Vector2d;

/// Represents a circle.
#[derive(Copy, Clone, PartialEq)]
pub struct Circle {
    pub position: Vector2d,
    pub radius: u32,
}

impl Debug for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

impl Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(position={}, radius={})", self.position, self.radius)
    }
}

impl Circle {
    pub fn position(&self) -> Vector2d {
        self.position
    }

    pub fn set_position(&mut self, value: &Vector2d) {
        self.position = value.clone();
    }

    pub fn radius(&self) -> u32 {
        self.radius
    }

    pub fn set_radius(&mut self, value: u32) {
        self.radius = value;
    }
}