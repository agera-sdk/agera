use std::sync::Arc;
use super::ecs::EntityTree;

pub struct App {
    entity_tree: Arc<EntityTree>,
    systems: Systems,
}

struct Systems {
    pub beginning: Vec<fn(Arc<Vec<Arc<EntityTree>>>)>,
    pub update: Vec<fn(Arc<Vec<Arc<EntityTree>>>)>,
    pub ending: Vec<fn(Arc<Vec<Arc<EntityTree>>>)>,
}