/*!
Work with the target platform.

# Native platforms

When building an Agera application for a native platform such as the Windows
operating system, Agera internally uses the following:

- Tokio runtime

# Browser platform

When building an Agera application for the browser, the following dependencies
are internally used:

- [wasm_bindgen](https://rustwasm.github.io/wasm-bindgen/)
- [wasm-bindgen-futures](https://crates.io/crates/wasm-bindgen-futures)
- [js-sys](https://crates.io/crates/js-sys)
- [web-sys](https://crates.io/crates/web-sys)
*/