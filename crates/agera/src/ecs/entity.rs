use crate::common::*;
use std::{
    any::{Any, TypeId},
    sync::{Arc, RwLock, Weak},
    hash::Hash,
};

type Component = Arc<dyn Any + Send + Sync>;

/// Represents an entity in the entity-component-system as a
/// reference-counted type.
pub struct Entity {
    inner: Arc<EntityInner>,
}

impl Hash for Entity {
    /// Hashes the entity by reference.
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        ByAddress(Arc::clone(&self.inner)).hash(state)
    }
}

impl PartialEq for Entity {
    /// Compares entities by reference.
    /// > **Note**: This method does not compare the entities by content.
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }
}

impl Eq for Entity {}

impl Clone for Entity {
    /// Clones the entity by reference.
    /// > **Note**: This method does not clone the entity by content.
    fn clone(&self) -> Self {
        Self { inner: Arc::clone(&self.inner) }
    }
}

impl Entity {
    pub fn new() -> Entity {
        Self {
            inner: Arc::new(EntityInner {
                parent: RwLock::new(default()),
                components: RwLock::new(vec![]),
                children: RwLock::new(vec![]),
            })
        }
    }

    /// Downgrades the entity reference into a weak reference.
    pub fn downgrade_ref(&self) -> WeakEntityRef {
        WeakEntityRef(Arc::downgrade(&self.inner))
    }

    /// Checks whether entity has a specified component.
    pub fn has<T>(&self) -> bool
        where T: Any + Send + Sync
    {
        self.get::<T>().is_some()
    }

    /// Retrieves a component from the entity.
    pub fn get<T>(&self) -> Option<Arc<T>>
        where T: Any + Send + Sync
    {
        for component in self.inner.components.read().unwrap().iter() {
            if let Ok(c) = Arc::downcast::<T>(Arc::clone(component)) {
                return Some(c);
            }
        }
        None
    }

    /// Overrides a component of the entity.
    pub fn set<T>(&self, value: T)
        where T: Any + Send + Sync
    {
        self.delete::<T>();
        self.inner.components.write().unwrap().push(Arc::new(value));
    }

    /// Deletes a component of the entity.
    /// Returns `true` if any component was deleted;
    /// otherwise returns `false`.
    pub fn delete<T>(&self) -> bool
        where T: Any + Send + Sync
    {
        let mut i = 0;
        let mut components = vec![];
        for component in self.inner.components.read().unwrap().iter() {
            components.push(Arc::clone(component));
        }
        for component in components {
            if Arc::downcast::<T>(Arc::clone(&component)).is_ok() {
                self.inner.components.write().unwrap().remove(i);
                return true;
            }
            i += 1;
        }
        false
    }
}

pub struct EntityInner {
    parent: RwLock<WeakEntityRef>,
    components: RwLock<Vec<Component>>,
    children: RwLock<Vec<Entity>>,
}

/// Represents a weak reference to an entity.
#[derive(Debug)]
pub struct WeakEntityRef(Weak<EntityInner>);

impl WeakEntityRef {
    /// Returns a `WeakEntityRef` reference that upgrades to no
    /// strong reference.
    pub fn empty() -> Self {
        Self(Weak::new())
    }

    /// Attempts to upgrade a weak reference into a strong reference.
    pub fn upgrade(&self) -> Option<Entity> {
        if let Some(r) = self.0.upgrade() { Some(Entity { inner: r }) } else { None }
    }
}

impl Default for WeakEntityRef {
    fn default() -> Self {
        Self::empty()
    }
}

impl PartialEq for WeakEntityRef {
    /// Compares entities by reference.
    fn eq(&self, other: &Self) -> bool {
        Weak::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for WeakEntityRef {}

impl Clone for WeakEntityRef {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}