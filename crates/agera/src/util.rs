/*!
Minor utilities.
*/

pub use ::bytes as bytes;
pub use ::serde as ser;
pub use ::serde_json as json;
pub use ::lazy_regex as regex;
pub use ::chrono as temporal;

pub mod uri;
pub mod literals;

/**
 * Returns the default value of a type.
 */
pub fn default<T: Default>() -> T {
    T::default()
}

pub use by_address::{ByAddress, ByThinAddress};

pub use once_cell::sync::Lazy;