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
            Despawn as purplelight_ecs_EntityDespawn,
            Get as purplelight_ecs_EntityGet,
            GetRef as purplelight_ecs_EntityGetRef,
            GetById as purplelight_ecs_EntityGetById,
            GetMutById as purplelight_ecs_EntityGetMutById,
            GetMut as purplelight_ecs_EntityGetMut,
            Location as purplelight_ecs_EntityLocation,
            Insert as purplelight_ecs_EntityInsert,
            Remove as purplelight_ecs_EntityRemove,
        },
        event::{Event, EventReader, EventWriter},
        hierarchy::{
            Children as EntityHierarchyChildren,
            Parent as EntityHierarchyParent,
            SpawnChild as EntityHierarchySpawnChild,
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