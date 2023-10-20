/*!
The entity-component-system pattern used by Purplelight
applications. It re-exports items from the [`bevy_ecs` crate](https://docs.rs/bevy_ecs/0.11)
and adds entity hierarchy.

# Components

Components are normal Rust structs. They are data stored in a `World` and specific
instances of Components correlate to Entities.

```
use purplelight::ecs::common::*;

#[derive(Component)]
struct Position { x: f32, y: f32 }
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