/*!
Minor utilities.
*/

pub use ::bytes as bytes;
pub use ::serde as ser;
pub use ::serde_json as json;
pub use ::lazy_regex as regex;
pub use ::chrono as temporal;

pub mod event_emitter;
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
    fn index_of(&self, value: &T) -> Option<usize> where T: Eq;

    /// Removes an element that meets the criteria `element == value`.
    fn remove_equals(&mut self, value: &T) -> bool  where T: Eq;
}

impl<T> VectorExtensions<T> for Vec<T> {
    fn index_of(&self, value: &T) -> Option<usize> where T: Eq {
        for i in 0..self.len() {
            if self[i] == *value {
                return Some(i);
            }
        }
        None
    }

    fn remove_equals(&mut self, value: &T) -> bool  where T: Eq {
        let i = self.index_of(value);
        if let Some(i) = i {
            self.remove(i);
            true
        } else {
            false
        }
    }
}

pub use late_substitution::LateSubstitution;