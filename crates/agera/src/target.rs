/*!
Work with the target platform.

# Native platforms

When building an Agera application for a native platform such as the Windows
operating system, Agera internally uses the following:

- Tokio runtime

The following items are available when building for native platforms:

- `agera::target::tokio` — Alias to the `tokio` crate.

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

if_native_target! {
    pub use tokio;
}

if_browser_target! {
    pub use wasm_bindgen as js_bindings;
    pub use wasm_bindgen_futures as js_futures;
    pub use js_sys as js;
    pub(crate) use web_sys as web;
}