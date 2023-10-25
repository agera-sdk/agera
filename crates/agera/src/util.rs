/*!
Utilities for the Rust language.

Items in this module are in general re-exported from other crates.
Most of `agera::util` is re-exported in `agera::common`, reducing
the need of importing them explicitly.
*/

pub use ::bytes as bytes;
pub use ::serde as ser;
pub use ::serde_json as json;
pub use ::lazy_regex as regex;
pub use ::chrono as temporal;
pub use ::file_paths as paths;

pub mod future;
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