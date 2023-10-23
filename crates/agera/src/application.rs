use crate::common::*;

static mut ROOT: Lazy<Entity> = Lazy::new(|| Entity::new());

/// The entity-component-system root entity.
pub fn root() -> Entity {
    unsafe { ROOT.clone() }
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

mod bootstrap;
pub use bootstrap::*;