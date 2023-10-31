use std::fmt::{Display, Debug};

use crate::geom::Vector2d;

/// Represents a rectangle by its position followed by its size.
#[derive(Copy, Clone, PartialEq)]
pub struct Rectangle(pub Vector2d, pub Vector2d);

impl Debug for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x={}, y={}, width={}, height={})", self.x(), self.y(), self.width(), self.height())
    }
}

impl Rectangle {
    pub fn position(&self) -> Vector2d {
        self.0.clone()
    }
    pub fn set_position(&mut self, value: &Vector2d) {
        self.0 = value.clone();
    }

    pub fn x(&self) -> f64 {
        self.position().x()
    }
    pub fn set_x(&mut self, value: f64) {
        self.0.set_x(value);
    }

    pub fn y(&self) -> f64 {
        self.position().y()
    }
    pub fn set_y(&mut self, value: f64) {
        self.0.set_y(value);
    }

    pub fn size(&self) -> Vector2d {
        self.1.clone()
    }

    pub fn set_size(&mut self, value: &Vector2d) {
        self.1 = value.clone();
    }

    pub fn width(&self) -> f64 {
        self.size().x()
    }
    pub fn set_width(&mut self, value: f64) {
        self.1.set_x(value);
    }

    pub fn height(&self) -> f64 {
        self.size().y()
    }
    pub fn set_height(&mut self, value: f64) {
        self.1.set_y(value);
    }
}

impl std::marker::StructuralEq for Rectangle {}