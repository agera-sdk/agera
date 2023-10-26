use futures::Future;
use crate::platforms::{if_native_platform, if_browser};

pub fn exec<F>(future: F)
where
    F: Future<Output = ()> + 'static,
{
    if_native_platform! {{
        crate::application::assert_bootstrapped!();
        tokio::task::spawn_local(future);
    }}
    if_browser! {{
        wasm_bindgen_futures::spawn_local(future);
    }}
}

/// Marks asynchronous code as `!Send`.
#[allow(unused)]
pub(crate) macro no_send {
    () => {
        futures::future::ready(std::marker::PhantomData::<*const ()>::default()).await;
    },
}

pub use futures::future::{
    join_all as all,
    select_all as race,
};