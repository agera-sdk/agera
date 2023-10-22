#![feature(decl_macro)]

pub mod application;
pub mod ecs;
pub mod file;
pub mod target;
pub mod timer;
pub mod util;

/// Commonly used items in Agera application code.
/// It is common to import this module into scope,
/// given that the Rust standard library does not cover
/// all use-cases:
/// 
/// ```
/// use agera::common::*;
/// ```
pub mod common {
    pub use std::collections::{
        HashMap,
        HashSet,
        BTreeMap,
        BTreeSet,
    };
    pub use crate::util::default;
    pub use crate::util::event_emitter::{
        EventEmitter,
        EventListener,
        event_listener,
    };

    pub use crate::util::bytes::{
        Bytes,
        BytesMut,
        Buf,
        BufMut,
    };
    pub use crate::util::ser::{
        Deserialize,
        Serialize,
    };
    pub use crate::util::regex::{
        lazy_regex,
        regex,
        regex_captures,
        regex_find,
        regex_is_match,
        regex_replace,
        regex_replace_all,
        Regex,
        Captures as RegexCaptures,
    };
    pub use crate::util::uri::*;
    pub use crate::util::literals::{
        btreemap,
        btreeset,
        hashmap,
        hashset,
    };
    pub use crate::util::{ByAddress, ByThinAddress};
    pub use crate::util::Lazy;
    pub use crate::util::temporal;
    pub use crate::util::future;
}