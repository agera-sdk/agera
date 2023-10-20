/*!
The entity-component-system pattern used by Purplelight applications.
It re-exports functionality from the [`bevy_ecs` crate](https://docs.rs/bevy_ecs/0.11)
and adds entity hierarchy and operations to Entities.

# Full documentation

Refer to the [`bevy_ecs` crate](https://docs.rs/bevy_ecs/0.11) for full documentation on the entity-component-system
pattern API.

# Entity operations

Importing `purplelight::ecs::common` into scope brings common Entity operations
as methods, which are not normally methods in the `bevy_ecs` crate. For example, importing this module
brings `.get::<Component>()` into scope.

# Entity hierarchy

Entities are created directly when adding a child entity to an existing entity. For example,
the following adds an empty `Entity` to the application's root entity:

```
use purplelight::ecs::common::*;
let child = purplelight::application::root().spawn_child(());
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
        change_detection::{
            Mut,
            Ref,
            MutUntyped,
        },
        component::{Component, ComponentId},
        entity::Entity,
        entity_operations::{
            Contains as purplelight_ecs_Entity_Contains,
            ContainsId as purplelight_ecs_Entity_ContainsId,
            ContainsTypeId as purplelight_ecs_Entity_ContainsTypeId,
            Despawn as purplelight_ecs_Entity_Despawn,
            Get as purplelight_ecs_Entity_Get,
            GetRef as purplelight_ecs_Entity_GetRef,
            GetById as purplelight_ecs_Entity_GetById,
            GetMutById as purplelight_ecs_Entity_GetMutById,
            GetMut as purplelight_ecs_Entity_GetMut,
            Location as purplelight_ecs_Entity_Location,
            Insert as purplelight_ecs_Entity_Insert,
            Remove as purplelight_ecs_Entity_Remove,
        },
        event::{Event, EventReader, EventWriter},
        hierarchy::{
            Children as purplelight_ecs_Entity_Children,
            DespawnChildren as purplelight_ecs_Entity_DespawnChildren,
            Parent as purplelight_ecs_Entity_Parent,
            SpawnChild as purplelight_ecs_Entity_SpawnChild,
        },
        path_operations::{
            Name as purplelight_ecs_Entity_Name,
            SetName as purplelight_ecs_Entity_SetName,
            ResolvePath as purplelight_ecs_Entity_ResolvePath,
        },
        query::{Added, Changed, With, Without},
        system::Query,
        world::World,
    };
}

pub mod hierarchy;
pub mod entity_operations;
pub mod path_operations;