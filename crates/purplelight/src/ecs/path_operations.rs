/*!
Path operations for Entities.
*/

use crate::ecs::common::*;

pub trait Name {
    fn name(&self) -> Option<String>;
}

impl Name for Entity {
    fn name(&self) -> Option<String> {
        crate::application::world().entity(*self).get::<NameComponent>().map(|c| c.0.clone())
    }
}

pub trait SetName {
    fn set_name(&self, name: Option<String>);
}

impl SetName for Entity {
    fn set_name(&self, name: Option<String>) {
        let mut entity = crate::application::world_mut().entity_mut(*self);
        if let Some(name) = name.clone() {
            entity.insert(NameComponent(name));
        } else if entity.contains::<NameComponent>() {
            entity.remove::<NameComponent>();
        }
    }
}

pub trait ResolvePath {
    /**
    Resolves an entity path. An entity path is resolved as follows:

    1. Let *segments* be the splitting of the path by the slash character (`/`).
    2. Let *r* be the initial entity.
    3. For every segment *s*:
        1. If `s == ".first"`, let *r* be the first child of *r* or otherwise `None`.
        2. If `s == ".last"`, let *r* be the last child of *r* or otherwise `None`.
        3. If `s == ".."`, let *r* be the parent of *r* or otherwise `None`.
        4. If *s* is non-empty, let *r* be a child of *r* such that `r.name() == s` or otherwise `None`.
    4. Return *r*
    */
    fn resolve_path(&self, path: &str) -> Option<Entity>;
}

impl ResolvePath for Entity {
    fn resolve_path(&self, path: &str) -> Option<Entity> {
        let segments = path.split('/');
        let mut r: Option<Entity> = Some(*self);
        for s in segments {
            if r.is_none() {
                break;
            }
            match s {
                ".first" => {
                    r = r.unwrap().children().first().map(|c| *c);
                },
                ".last" => {
                    r = r.unwrap().children().last().map(|c| *c);
                },
                ".." => {
                    r = r.unwrap().parent();
                },
                "" => {
                    // Empty
                },
                _ => {
                    r = r.unwrap().children().iter().find(|c| c.name().as_ref().map(|cn| cn.as_ref()) == Some(s)).map(|c| *c);
                },
            }
        }
        r
    }
}

#[derive(Component)]
struct NameComponent(String);