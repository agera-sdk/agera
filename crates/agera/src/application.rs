use crate::ecs::common::*;
use crate::common::*;

static mut ROOT: Entity = Entity::PLACEHOLDER;

/// The entity-component-system root entity.
pub fn root() -> Entity {
    unsafe { ROOT }
}

static mut WORLD: Lazy<World> = Lazy::new(|| {
    let mut world = World::new();
    unsafe { ROOT = world.spawn(()).id(); }
    world
});

/// The entity-component-system world.
pub fn world() -> &'static Lazy<World> {
    unsafe { &WORLD }
}

/// The entity-component-system world, as a mutable borrow.
pub fn world_mut() -> &'static mut Lazy<World> {
    unsafe { &mut WORLD }
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