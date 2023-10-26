use crate::platforms::{if_native_platform, if_browser};

if_native_platform! {
    pub mod native;
    pub use native::*;
}

if_browser! {
    pub mod browser;
    pub use browser::*;
}