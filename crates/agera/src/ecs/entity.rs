use crate::common::*;
use std::{
    any::Any,
    sync::Arc,
};

type Component = Box<dyn Any + Send + Sync>;

/// Represents an entity in the entity-component-system as a
/// reference-counted type.
pub struct Entity {
    inner: Arc<EntityInner>,
}

pub struct EntityInner {
    components: Vec<Component>,
    children: Vec<Entity>,
}