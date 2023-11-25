/*!
Utilities APIs.
*/

pub use ::bytes as bytes;
pub use ::serde as ser;
pub use ::serde_json as json;
pub use ::lazy_regex as regex;
pub use ::chrono as temporal;
pub use ::file_paths as paths;

pub mod future;
pub mod html;
pub mod uri;
pub mod literals;

mod color;
pub use self::color::*;

pub use late_format::LateFormat;

pub use by_address::ByAddress;

pub use once_cell::sync::Lazy;

pub use ::cfg_if::cfg_if;

mod vector_extensions;
pub use self::vector_extensions::*;

/// Returns the default value of a type.
pub fn default<T: Default>() -> T {
    T::default()
}

pub use with_literal::with;