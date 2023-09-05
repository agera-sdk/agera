#![feature(decl_macro)]

pub use ::bytes as bytes;
pub mod future {
    use futures::Future;

    /// The `future_race` function takes an iterable of futures as input and returns
    /// a single [`Future`]. The returned future completes with
    /// a group (_v_, _i_), where _v_ is the output from the first
    /// completed future and _i_ is the index of the first completed future
    /// from the given iterator.
    /// 
    /// # Exceptions
    /// 
    /// Panics if the iterator specified contains no futures.
    /// 
    /// # Example
    /// 
    /// ```
    /// # use purplelight::lang::future::future_race;
    /// # async fn f() {
    /// let (value, index) = future_race(list_of_futures).await;
    /// # }
    /// ```
    /// 
    pub async fn future_race<I, IteratorFuture>(iterable: I) -> (IteratorFuture::Output, usize)
    where
        I: IntoIterator<Item = IteratorFuture>,
        IteratorFuture: Future + Unpin,
    {
        let (v, i, _) = futures::future::select_all(iterable).await;
        (v, i)
    }

    pub async fn future_all<I, IteratorFuture>(iterable: I) -> Vec<IteratorFuture::Output>
    where
        I: IntoIterator<Item = IteratorFuture>,
        IteratorFuture: Future + Unpin,
    {
        futures::future::join_all(iterable).await
    }

    /// Marks a future as `!Send`.
    pub macro not_sendable_async {
        () => {
            ::futures::future::ready(std::marker::PhantomData::<*const ()>::default()).await;
        },
    }
}
pub mod regexp;
pub use ::serde as ser;
pub use ::serde_json as json;
pub mod uri;

pub mod prelude {
    pub use super::bytes::{Bytes, BytesMut};
    pub use super::future::{
        future_race,
        future_all,
        not_sendable_async,
    };
    pub use super::regexp::{
        LazyRegexp,
        Regexp,
        RegexpCaptures,
        regexp,
        lazy_regexp,
        regexp_captures,
        regexp_find,
        regexp_is_match,
        regexp_replace,
        regexp_replace_all,
    };
    pub use std::collections::{HashMap, HashSet};
    pub use super::{ser, json};

    pub fn default<T: Default>() -> T {
        Default::default()
    }

    pub use maplit::{
        hashmap,
        hashset,
        btreemap,
        btreeset,
    };

    pub use super::uri::{
        encode_uri,
        encode_uri_component,
        decode_uri,
        decode_uri_component,
    };
}