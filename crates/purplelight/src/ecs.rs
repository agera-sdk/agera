/*!
The entity-component-system pattern used by Purplelight applications.
It re-exports functionality from the [`bevy_ecs` crate](https://docs.rs/bevy_ecs/0.11)
and adds entity hierarchy.

# Entity hierarchy

Entities are created directly when adding a child entity to an existing entity. For example,
the following adds an empty `Entity` to the application's root entity:

```
use purplelight::ecs::common::*;
let child = purplelight::application().root().spawn_child(());
```
*/

pub use ::bevy_ecs::{
    archetype,
    bundle,
    change_detection,
    component,
    entity,
    event,
    query,
    reflect,
    removal_detection,
    schedule,
    storage,
    system,
    world,
};

/// Commonly used ECS items.
pub mod common {
    pub use super::{
        bundle::Bundle,
        component::{Component, ComponentId},
        entity::Entity,
        event::{Event, EventReader, EventWriter},
        query::{Added, Changed, With, Without},
        system::Query,
        world::World,
    };
}