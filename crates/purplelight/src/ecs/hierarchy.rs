use crate::ecs::{
    bundle::Bundle,
    common::*,
    world::EntityMut,
};

pub trait SpawnChildEntity {
    fn spawn_child(&self, bundle: impl Bundle) -> EntityMut<'static>;
}

impl SpawnChildEntity for Entity {
    fn spawn_child(&self, bundle: impl Bundle) -> EntityMut<'static> {
        let mut child = crate::application::world_mut().spawn(bundle);
        child.insert(EntityParent(*self));
        child
    }
}

#[derive(Component)]
pub(crate) struct EntityParent(Entity);