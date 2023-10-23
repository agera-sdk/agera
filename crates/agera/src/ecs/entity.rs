use crate::common::*;
use std::{
    any::Any,
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
                name: RwLock::new(None),
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
    
    pub fn parent(&self) -> Option<Entity> {
        self.inner.parent.read().unwrap().upgrade()
    }

    pub fn children(&self) -> Vec<Entity> {
        let mut c = vec![];
        for child in self.inner.children.read().unwrap().iter() {
            c.push(child.clone());
        }
        c
    }

    pub fn child_at(&self, index: usize) -> Option<Entity> {
        if index < self.num_children() { Some(self.inner.children.read().unwrap()[index].clone()) } else { None }
    }

    /// Returns the number of children.
    pub fn num_children(&self) -> usize {
        self.inner.children.read().unwrap().len()
    }

    /// Adds a child entity to the end of the children collection.
    /// If `child` is already child of an entity, it is removed and then added
    /// as part of this entity.
    pub fn add_child(&self, child: &Entity) {
        child.remove_from_parent();
        *child.inner.parent.write().unwrap() = self.downgrade_ref();
        self.inner.children.write().unwrap().push(child.clone());
    }

    /// Adds a child entity at the index `index` of the children collection.
    /// If `child` is already child of an entity, it is removed and then added
    /// as part of this entity.
    /// 
    /// # Panics
    /// 
    /// This method panics if `index` is out of bounds.
    pub fn add_child_at(&self, index: usize, child: &Entity) {
        child.remove_from_parent();
        assert!(index < self.num_children(), "Specified index is out of bounds.");
        *child.inner.parent.write().unwrap() = self.downgrade_ref();
        self.inner.children.write().unwrap().insert(index, child.clone());
    }

    /// Swaps two children.
    /// 
    /// # Panics
    /// 
    /// Panics if any of the specified entities is not part of the entity.
    pub fn swap_children(&self, child_1: &Entity, child_2: &Entity) {
        let indices = [self.inner.children.read().unwrap().index_of(child_1), self.inner.children.read().unwrap().index_of(child_2)];
        assert!(indices.iter().all(|i| i.is_some()), "Some of the specified indices are out of bounds.");
        self.inner.children.write().unwrap().swap(indices[0].unwrap(), indices[1].unwrap());
    }

    /// Swaps two children.
    /// 
    /// # Panics
    /// 
    /// Panics if any of the specified indices is out of bounds.
    pub fn swap_children_by_indices(&self, child_1: usize, child_2: usize) {
        assert!([child_1, child_2].iter().all(|&i| i < self.num_children()), "Some of the specified indices are out of bounds.");
        self.inner.children.write().unwrap().swap(child_1, child_2);
    }

    /// Removes a child. Returns `true` if the child has been removed, or `false` otherwise.
    pub fn remove_child(&self, child: &Entity) -> bool {
        let i = self.inner.children.read().unwrap().index_of(child);
        if let Some(i) = i {
            self.inner.children.write().unwrap().remove(i);
            *child.inner.parent.write().unwrap() = default();
            true
        } else {
            false
        }
    }

    /// Removes all children entities from the entity.
    pub fn remove_children(&self) {
        for child in self.children() {
            *child.inner.parent.write().unwrap() = default();
        }
        self.inner.children.write().unwrap().clear();
    }

    /// Removes the entity from its parent. Returns `true` if the child has been removed, or `false` otherwise.
    pub fn remove_from_parent(&self) -> bool {
        if let Some(p) = self.parent() { p.remove_child(self) } else { false }
    }

    /// The name of the entity as used in entity paths.
    pub fn name(&self) -> Option<String> {
        self.inner.name.read().unwrap().clone()
    }

    /// The name of the entity as used in entity paths.
    pub fn set_name(&self, name: Option<String>) {
        *self.inner.name.write().unwrap() = name;
    }

    /**
    Resolves an entity path. An entity path is resolved as follows:

    1. Let *segments* be the splitting of the path by the slash character (`/`).
    2. Let *r* be the initial entity.
    3. For every segment *s*:
        1. If `s == ".first"`, let *r* be the first child of *r* or otherwise `None`.
        2. If `s == ".last"`, let *r* be the last child of *r* or otherwise `None`.
        3. If `s == ".."`, let *r* be the parent of *r* or otherwise `None`.
        4. If *s* is non-empty, let *r* be a child of *r* such that `child.name() == s` or otherwise `None`.
    4. Return *r*
    */
    pub fn resolve_path(&self, path: &str) -> Option<Entity> {
        let segments = path.split('/');
        let mut r: Option<Entity> = Some(self.clone());
        for s in segments {
            if r.is_none() {
                break;
            }
            match s {
                ".first" => {
                    r = r.unwrap().children().first().map(|c| c.clone());
                },
                ".last" => {
                    r = r.unwrap().children().last().map(|c| c.clone());
                },
                ".." => {
                    r = r.unwrap().parent();
                },
                "" => {
                    // Empty
                },
                _ => {
                    r = r.unwrap().children().iter().find(|c| c.name().as_ref().map(|cn| cn.as_ref()) == Some(s)).map(|c| c.clone());
                },
            }
        }
        r
    }
}

pub struct EntityInner {
    name: RwLock<Option<String>>,
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