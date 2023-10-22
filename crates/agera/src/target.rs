/*!
Work with the target platform.

# Native platforms

The following items are available when building for native platforms:

- `agera::target::tokio` — Alias to the `tokio` crate, an asynchronous runtime for Rust.

# Android platform

The folllowing items are available when building an Agera application
for the Android operating system:

- `agera::target::activity` — Alias to the [`android_activity`](https://crates.io/crates/android-activity) crate.
- `agera::target::jni` — Alias to the [`jni`](https://crates.io/crates/jni) crate.
- `agera::target::application()` — Returns an `agera::target::activity::AndroidApp` value
providing access to the Android application.

# Browser

When building an Agera application for the browser, the following dependencies
are internally used:

- [wasm_bindgen](https://rustwasm.github.io/wasm-bindgen/)
- [wasm-bindgen-futures](https://crates.io/crates/wasm-bindgen-futures)
- [js-sys](https://crates.io/crates/js-sys)
- [web-sys](https://crates.io/crates/web-sys)

The following items are available when building for the browser:

- `agera::target::js_bindings` — Alias to the `wasm_bindgen` crate.
- `agera::target::js_futures` — Alias to the `wasm_bindgen_futures` crate.
- `agera::target::js` — Alias to the `js_sys` crate.
*/

use crate::common::*;
use std::sync::RwLock;

/// Expands an item solely if the build target is a native platform.
pub macro if_native_target {
    ($it:block) => {
        #[cfg(not(target_arch = "wasm32"))]
        $it
    },
    ($($it:item)+) => {
        $(
            #[cfg(not(target_arch = "wasm32"))]
            $it
        )+
    },
}

/// Expands an item solely if the build target is the browser.
pub macro if_browser_target {
    ($it:block) => {
        #[cfg(target_arch = "wasm32")]
        $it
    },
    ($($it:item)+) => {
        $(
            #[cfg(target_arch = "wasm32")]
            $it
        )+
    },
}

pub(crate) macro unsupported_platform {
    () => {
        panic!("Unsupported platform");
    },
}

if_native_target! {
    pub use tokio;
}

#[cfg(target_os = "android")]
pub use ::android_activity as activity;

#[cfg(target_os = "android")]
pub use ::jni;

#[cfg(target_os = "android")]
#[doc(hidden)]
pub static APPLICATION: Lazy<RwLock<Option<activity::AndroidApp>>> = Lazy::new(|| RwLock::new(None));

#[cfg(target_os = "android")]
pub fn application() -> activity::AndroidApp {
    APPLICATION.read().unwrap().as_ref().unwrap().clone()
}

if_browser_target! {
    pub use wasm_bindgen as js_bindings;
    pub use wasm_bindgen_futures as js_futures;
    pub use js_sys as js;
    pub(crate) use web_sys as web;
}