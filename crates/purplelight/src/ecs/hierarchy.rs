//! Hierarchy traits for Entities.

use crate::ecs::{
    bundle::Bundle,
    common::*,
    world::EntityMut,
};

pub trait SpawnChild {
    fn spawn_child(&self, bundle: impl Bundle) -> EntityMut<'static>;
}

impl SpawnChild for Entity {
    fn spawn_child(&self, bundle: impl Bundle) -> EntityMut<'static> {
        let mut child = crate::application::world_mut().spawn(bundle);
        child.insert(ParentComponent(*self));
        child
    }
}

pub trait Parent {
    fn parent(&self) -> Option<Entity>;
}

impl Parent for Entity {
    fn parent(&self) -> Option<Entity> {
        crate::application::world().get::<ParentComponent>(*self).map(|c| c.0)
    }
}

pub trait Children {
    fn children(&self) -> Vec<Entity>;
}

impl Children for Entity {
    fn children(&self) -> Vec<Entity> {
        let mut r: Vec<Entity> = vec![];
        for ent in crate::application::world().iter_entities() {
            let Some(c) = ent.get::<ParentComponent>() else {
                break;
            };
            if c.0 == *self {
                r.push(ent.id());
            }
        }
        r
    }
}

pub trait DespawnChildren {
    /// Despawns all children entities from an entity.
    fn despawn_children(&self);
}

impl DespawnChildren for Entity {
    fn despawn_children(&self) {
        for child in self.children() {
            child.despawn();
        }
    }
}

#[derive(Component)]
struct ParentComponent(Entity);