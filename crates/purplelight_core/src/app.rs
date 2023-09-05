use std::sync::Arc;
use super::ecs::EntityTree;

pub struct App {
    systems: Vec<fn(Arc<Vec<Arc<EntityTree>>>)>,
}