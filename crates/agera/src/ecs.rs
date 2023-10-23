/*!
The entity-component-system pattern used by Agera applications.

# Entity

An entity consits of a set of components and an entity can have other
children entities.

```rust
use agera::common::*;

let something = Entity::new();

// Set a component
something.set::<f64>(10);

// `Option<Arc<f64>>`
something.get::<f64>();

// `agera::application::root()` is the root `Entity` of the
// application.
agera::application::root().add_child(something);
```

# System

At the present, systems from the entity-component-system pattern
are not concrete in Agera and there are no facilities to query
entities by components.
*/

mod entity;
pub use entity::*;