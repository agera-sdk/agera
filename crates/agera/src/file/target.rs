use crate::target::{if_browser_target, if_native_target};

if_native_target! {
    pub mod native;
}

if_browser_target! {
    pub mod browser;
}