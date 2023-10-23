/*!
The entity-component-system pattern used by Agera applications.
It re-exports functionality from the [`bevy_ecs` crate](https://docs.rs/bevy_ecs/0.11)
and adds entity hierarchy and operations to Entities.

# Full documentation

Refer to the [`bevy_ecs` crate](https://docs.rs/bevy_ecs/0.11) for full documentation on the entity-component-system
pattern API.

# Entity operations

Importing `agera::ecs::common` into scope brings common Entity operations
as methods, which are not normally methods in the `bevy_ecs` crate. For example, importing this module
brings `.get::<Component>()` into scope.

# Entity hierarchy

Entities are created directly when adding a child entity to an existing entity. For example,
the following adds an empty `Entity` to the application's root entity:

```
use agera::ecs::common::*;
let child = agera::application::root().spawn_child(());
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

/// Commonly used entity-component-system items.
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
            Contains as agera_Entity_Contains,
            ContainsId as agera_Entity_ContainsId,
            ContainsTypeId as agera_Entity_ContainsTypeId,
            Despawn as agera_Entity_Despawn,
            Get as agera_Entity_Get,
            GetRef as agera_Entity_GetRef,
            GetById as agera_Entity_GetById,
            GetMutById as agera_Entity_GetMutById,
            GetMut as agera_Entity_GetMut,
            Location as agera_Entity_Location,
            Insert as agera_Entity_Insert,
            Remove as agera_Entity_Remove,
        },
        event::{Event, EventReader, EventWriter},
        hierarchy::{
            Children as agera_Entity_Children,
            DespawnChildren as agera_Entity_DespawnChildren,
            Parent as agera_Entity_Parent,
            SpawnChild as agera_Entity_SpawnChild,
        },
        path_operations::{
            Name as agera_Entity_Name,
            SetName as agera_Entity_SetName,
            ResolvePath as agera_Entity_ResolvePath,
        },
        query::{Added, Changed, With, Without},
        system::Query,
        world::World,
    };
}

pub mod hierarchy;
pub mod entity_operations;
pub mod path_operations;