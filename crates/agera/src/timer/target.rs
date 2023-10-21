use crate::target::{if_native_target, if_browser_target};

if_native_target! {
    pub mod native;
    pub use native::*;
}

if_browser_target! {
    pub mod browser;
    pub use browser::*;
}