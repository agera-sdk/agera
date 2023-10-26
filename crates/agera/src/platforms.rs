/*!
Work with the target platform.

# Native platforms

The following items are available when building for native platforms:

- `agera::platforms::tokio` — Alias to the `tokio` crate, an asynchronous runtime for Rust.

# Android platform

The folllowing items are available when building an Agera application
for the Android operating system:

- `agera::platforms::activity` — Alias to the [`android_activity`](https://crates.io/crates/android-activity) crate.
- `agera::platforms::jni` — Alias to the [`jni`](https://crates.io/crates/jni) crate.
- `agera::platforms::application()` — Returns an `agera::platforms::activity::AndroidApp` value
providing access to the Android application.

# Browser

When building an Agera application for the browser, the following dependencies
are internally used:

- [wasm_bindgen](https://rustwasm.github.io/wasm-bindgen/)
- [wasm-bindgen-futures](https://crates.io/crates/wasm-bindgen-futures)
- [js-sys](https://crates.io/crates/js-sys)

The following items are available when building for the browser:

- `agera::platforms::js_bindings` — Alias to the `wasm_bindgen` crate.
- `agera::platforms::js_futures` — Alias to the `wasm_bindgen_futures` crate.
- `agera::platforms::js` — Alias to the `js_sys` crate.
*/

#[allow(unused)]
use crate::common::*;

#[allow(unused)]
use std::sync::RwLock;

/// Expands an item solely if the build target is a native platform.
pub macro if_native_platform {
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
pub macro if_browser {
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

#[allow(unused)]
pub(crate) macro unsupported_platform {
    () => {
        panic!("Unsupported platform");
    },
}

if_native_platform! {
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

if_browser! {
    pub use wasm_bindgen as js_bindings;
    pub use wasm_bindgen_futures as js_futures;
    pub use js_sys as js;
}