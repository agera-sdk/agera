use std::any::TypeId;
use crate::ecs::common::*;
use bevy_ecs::{ptr::Ptr, entity::EntityLocation};

pub trait Despawn {
    fn despawn(&self) -> bool;
}

impl Despawn for Entity {
    fn despawn(&self) -> bool {
        crate::application::world_mut().despawn(*self)
    }
}

pub trait Get {
    fn get<T: Component>(&self) -> Option<&T>;
}

impl Get for Entity {
    fn get<T: Component>(&self) -> Option<&T> {
        crate::application::world().get(*self)
    }
}

pub trait GetRef {
    fn get_ref<T: Component>(&self) -> Option<Ref<T>>;
}

impl GetRef for Entity {
    fn get_ref<T: Component>(&self) -> Option<Ref<T>> {
        crate::application::world().entity(*self).get_ref()
    }
}

pub trait GetById {
    fn get_by_id(&self, component_id: ComponentId) -> Option<Ptr<'static>>;
}

impl GetById for Entity {
    fn get_by_id(&self, component_id: ComponentId) -> Option<Ptr<'static>> {
        crate::application::world().get_by_id(*self, component_id)
    }
}

pub trait GetMutById {
    fn get_mut_by_id(&self, component_id: ComponentId) -> Option<MutUntyped<'static>>;
}

impl GetMutById for Entity {
    fn get_mut_by_id(&self, component_id: ComponentId) -> Option<MutUntyped<'static>> {
        crate::application::world_mut().get_mut_by_id(*self, component_id)
    }
}

pub trait GetMut {
    fn get_mut<T: Component>(&self) -> Option<Mut<T>>;
}

impl GetMut for Entity {
    fn get_mut<T: Component>(&self) -> Option<Mut<T>> {
        crate::application::world_mut().get_mut(*self)
    }
}

pub trait Contains {
    fn contains<T: Component>(&self) -> bool;
}

impl Contains for Entity {
    fn contains<T: Component>(&self) -> bool {
        crate::application::world().entity(*self).contains::<T>()
    }
}

pub trait ContainsId {
    fn contains_id(&self, id: ComponentId) -> bool;
}

impl ContainsId for Entity {
    fn contains_id(&self, id: ComponentId) -> bool {
        crate::application::world().entity(*self).contains_id(id)
    }
}

pub trait ContainsTypeId {
    fn contains_type_id(&self, id: TypeId) -> bool;
}

impl ContainsTypeId for Entity {
    fn contains_type_id(&self, id: TypeId) -> bool {
        crate::application::world().entity(*self).contains_type_id(id)
    }
}

pub trait Location {
    fn location(&self) -> EntityLocation;
}

impl Location for Entity {
    fn location(&self) -> EntityLocation {
        crate::application::world().entity(*self).location()
    }
}

pub trait Insert {
    fn insert(&self, bundle: impl Bundle) -> &Self;
}

impl Insert for Entity {
    fn insert(&self, bundle: impl Bundle) -> &Self {
        crate::application::world_mut().entity_mut(*self).insert(bundle);
        self
    }
}

pub trait Remove {
    fn remove<T: Bundle>(&self) -> &Self;
}

impl Remove for Entity {
    fn remove<T: Bundle>(&self) -> &Self {
        crate::application::world_mut().entity_mut(*self).remove::<T>();
        self
    }
}

pub trait Take {
    fn take<T: Bundle>(&self) -> Option<T>;
}

impl Take for Entity {
    fn take<T: Bundle>(&self) -> Option<T> {
        crate::application::world_mut().entity_mut(*self).take::<T>()
    }
}