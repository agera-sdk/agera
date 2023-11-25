/*!
Common items in Rust programs.

```
use agera::common::*;
```
*/

pub use std::collections::{
    HashMap,
    HashSet,
    BTreeMap,
    BTreeSet,
};

pub use crate::events::{
    EventEmitter,
    EventListener,
};

pub use crate::util::{default, with};

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
pub use crate::util::{
    uri::*,
    html::{
        escape_html,
        unescape_html,
    },
};
pub use crate::util::literals::{
    btreemap,
    btreeset,
    hashmap,
    hashset,
};

pub use crate::util::{
    ByAddress,

    Lazy,
    
    cfg_if,

    future,

    json,

    temporal,

    VectorExtensions,

    LateFormat,
};