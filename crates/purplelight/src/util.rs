/*!
Minor utilities.
*/

pub use ::bytes as bytes;
pub use ::serde as ser;
pub use ::serde_json as json;
pub use ::by_address as by_address;

pub mod uri;
pub mod literals;

/**
 * Returns the default value of a type.
 */
pub fn default<T: Default>() -> T {
    T::default()
}

pub use bytes::{
    Bytes,
    BytesMut,
    Buf,
    BufMut,
};
pub use ser::{
    Deserialize,
    Serialize,
};
pub use lazy_regex::{
    lazy_regex,
    regex,
    regex_captures,
    regex_find,
    regex_is_match,
    regex_replace,
    regex_replace_all,
    Lazy,
    Regex,
    Captures,
};
pub use uri::*;
pub use literals::{
    btreemap,
    btreeset,
    hashmap,
    hashset,
};
pub use by_address::{ByAddress, ByThinAddress};