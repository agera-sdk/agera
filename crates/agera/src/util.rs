/*!
Miscellaneous utilities.

Items in this module are in general re-exported from other crates
in crates.io. Most of `agera::util` is re-exported in `agera::common`, reducing
the need of importing them explicitly.
*/

use std::{fmt::{Debug, Display}, str::FromStr};

pub use ::bytes as bytes;
pub use ::serde as ser;
pub use ::serde_json as json;
pub use ::lazy_regex as regex;
pub use ::chrono as temporal;
pub use ::file_paths as paths;

pub mod future;
pub mod uri;
pub mod literals;

/// Returns the default value of a type.
pub fn default<T: Default>() -> T {
    T::default()
}

pub use by_address::{ByAddress, ByThinAddress};

pub use once_cell::sync::Lazy;

pub use ::cfg_if::cfg_if;

/// Provides additional methods for the standard `Vec<T>` type.
pub trait VectorExtensions<T> {
    /// Finds the index of a value.
    fn index_of(&self, value: &T) -> Option<usize> where T: PartialEq;

    /// Finds the index of a value starting from `start_index`.
    fn index_of_from(&self, value: &T, start_index: usize) -> Option<usize> where T: PartialEq;

    /// Removes an element that meets the criteria `element == value`.
    fn remove_equals(&mut self, value: &T) -> bool  where T: PartialEq;
}

impl<T> VectorExtensions<T> for Vec<T> {
    fn index_of(&self, value: &T) -> Option<usize> where T: PartialEq {
        for i in 0..self.len() {
            if self[i] == *value {
                return Some(i);
            }
        }
        None
    }

    fn index_of_from(&self, value: &T, start_index: usize) -> Option<usize> where T: PartialEq {
        if start_index >= self.len() {
            return None;
        }
        for i in start_index..self.len() {
            if self[i] == *value {
                return Some(i);
            }
        }
        None
    }

    fn remove_equals(&mut self, value: &T) -> bool  where T: PartialEq {
        let i = self.index_of(value);
        if let Some(i) = i {
            self.remove(i);
            true
        } else {
            false
        }
    }
}

pub use late_format::LateFormat;

/// Represents a color containing red, green, blue and alpha channels.
///
/// # Constructing Color from a string
/// 
/// The `Color` type can be constructed from a [CSS Color Module Level 4](https://www.w3.org/TR/css-color-4/)
/// string of the following forms:
/// 
/// * hexadecimal notations in the forms `#xxx` and `#xxxxxx`;
/// * `rgb()` and `rgba()`;
/// * `hsl()` and `hsla()`;
/// * `hwb()`;
/// * named colors and the `transparent` keyword.
/// 
/// Example parsing a string into a `Color` object:
/// 
/// ```
/// use agera::util::Color;
/// let color: Color = "rgba".parse().unwrap()
/// ```
#[derive(Clone, Copy, PartialEq)]
pub struct Color(css_color::Srgb);

impl Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(red={}, green={}, blue={}, alpha={})", self.0.red, self.0.green, self.0.blue, self.0.alpha)
    }
}

impl std::marker::StructuralEq for Color {}

impl Color {
    /// Constructs a `Color` from individual components.
    pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self(css_color::Srgb::new(red, green, blue, alpha))
    }

    /// Channel value in the range between 0 and 1.
    pub fn red(&self) -> f32 {
        self.0.red
    }

    /// Channel value in the range between 0 and 1.
    pub fn set_red(&mut self, value: f32) {
        self.0.red = value;
    }

    /// Channel value in the range between 0 and 1.
    pub fn green(&self) -> f32 {
        self.0.green
    }

    /// Channel value in the range between 0 and 1.
    pub fn set_green(&mut self, value: f32) {
        self.0.green = value;
    }

    /// Channel value in the range between 0 and 1.
    pub fn blue(&self) -> f32 {
        self.0.blue
    }

    /// Channel value in the range between 0 and 1.
    pub fn set_blue(&mut self, value: f32) {
        self.0.blue = value;
    }

    /// Channel value in the range between 0 and 1.
    pub fn alpha(self) -> f32 {
        self.0.alpha
    }

    /// Channel value in the range between 0 and 1.
    pub fn set_alpha(&mut self, value: f32) {
        self.0.alpha = value;
    }
}

impl FromStr for Color {
    type Err = ColorSyntaxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse().map_err(|_| ColorSyntaxError)?))
    }
}

/// Represents red, green, blue and alpha offsets in the range between -255 and 255.
#[derive(Clone)]
pub struct ColorOffsets {
    red: i32,
    green: i32,
    blue: i32,
    alpha: i32,
}

impl ColorOffsets {
    /// Constructs a `ColorOffsets` value from components in the range
    /// between -255 and 255.
    pub fn new(red: i32, green: i32, blue: i32, alpha: i32) -> Self {
        Self { red, green, blue, alpha }
    }

    /// Channel value in the range between -255 and 255.
    pub fn red(&self) -> i32 {
        self.red
    }

    /// Channel value in the range between -255 and 255.
    pub fn set_red(&mut self, value: i32) {
        self.red = value;
    }

    /// Channel value in the range between -255 and 255.
    pub fn green(&self) -> i32 {
        self.green
    }

    /// Channel value in the range between -255 and 255.
    pub fn set_green(&mut self, value: i32) {
        self.green = value;
    }

    /// Channel value in the range between -255 and 255.
    pub fn blue(&self) -> i32 {
        self.blue
    }

    /// Channel value in the range between -255 and 255.
    pub fn set_blue(&mut self, value: i32) {
        self.blue = value;
    }

    /// Channel value in the range between -255 and 255.
    pub fn alpha(self) -> i32 {
        self.alpha
    }

    /// Channel value in the range between -255 and 255.
    pub fn set_alpha(&mut self, value: i32) {
        self.alpha = value;
    }
}

impl Default for ColorOffsets {
    fn default() -> Self {
        Self::new(0, 0, 0, 0)
    }
}

impl std::ops::Add for ColorOffsets {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
            alpha: self.alpha + rhs.alpha,
        }
    }
}

impl std::ops::AddAssign for ColorOffsets {
    fn add_assign(&mut self, rhs: Self) {
        self.red += rhs.red;
        self.green += rhs.green;
        self.blue += rhs.blue;
        self.alpha += rhs.alpha;
    }
}

/// Error resulted from parsing a `Color` from a string.
#[derive(Debug)]
pub struct ColorSyntaxError;

impl Display for ColorSyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error when parsing `Color` from string")
    }
}