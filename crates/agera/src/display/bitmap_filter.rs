use crate::{
    geom::*,
    util::{Color, ColorOffsets}
};

/// A bitmap filter applicable to display objects.
#[derive(Clone)]
pub enum BitmapFilter {
    /// A color filter.
    Color {
        /// The amounts to multiply the red, green, blue and alpha
        /// channels.
        multiplier: Color,
        /// The amount to add to the red, green blue and alpha
        /// channels after multiplying them.
        offsets: ColorOffsets,
    },
    /// A blur filter.
    Blur {
        /// The amounts of horizontal and vertical blur.
        blur: Vector2d,
        /// The number of times to perform the blur. The constants
        /// `BitmapFilter::LOW_QUALITY`, `BitmapFilter::MEDIUM_QUALITY`, and `BitmapFilter::HIGH_QUALITY`
        /// may be enough for most applications.
        quality: u32,
    },
    /// A drop shadow filter.
    DropShadow {
        color: Color,
        offset: Vector2d,
        spread: u32,
        blur: Vector2d,
    },
}

impl BitmapFilter {
    pub const LOW_QUALITY: u32 = 1;
    pub const MEDIUM_QUALITY: u32 = 2;
    pub const HIGH_QUALITY: u32 = 3;
}