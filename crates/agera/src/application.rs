use std::sync::Arc;
use crate::{common::*, display::*};

static mut WINDOW: Lazy<Arc<Window>> = Lazy::new(|| Arc::new(Window {
    root: DisplayObject::new(),
}));

/// The main window of the application.
pub fn window() -> Arc<Window> {
    unsafe { Arc::clone(&WINDOW) }
}

/// The root display object of the application's main window.
pub fn root() -> DisplayObject {
    unsafe { WINDOW.root() }
}

/// *Internal property.*
#[doc(hidden)]
#[allow(non_upper_case_globals)]
pub static mut __agera_ID: Option<&'static str> = None;

/// The application ID, as a Java package identifier.
pub fn id() -> String {
    assert_bootstrapped!();
    unsafe { __agera_ID.unwrap().to_owned() }
}

pub const fn is_browser_application() -> bool {
    cfg_if! { if #[cfg(target_arch = "wasm32")] { true } else { false } }
}

pub const fn is_native_application() -> bool {
    cfg_if! { if #[cfg(target_arch = "wasm32")] { false } else { true } }
}

mod bootstrap;
pub use bootstrap::*;

mod window;
pub use window::*;