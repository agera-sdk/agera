use futures::Future;
use crate::target::{if_native_target, if_browser_target};

pub fn exec<F>(future: F)
where
    F: Future<Output = ()> + 'static,
{
    if_native_target! {{
        crate::application::assert_bootstrapped!();
        tokio::task::spawn_local(future);
    }}
    if_browser_target! {{
        wasm_bindgen_futures::spawn_local(future);
    }}
}

/// Marks asynchronous code as `!Send`.
pub(crate) macro no_send {
    () => {
        futures::future::ready(std::marker::PhantomData::<*const ()>::default()).await;
    },
}

pub use futures::future::{
    join_all as all,
    select_all as race,
};