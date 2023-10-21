# Agera SDK

Develop rich internet applications robustly through Agera SDK.

The Agera SDK is being built in the Rust language. However, unlike other Rust frameworks, it does not require knowledge of the Rust ecosystem: the framework bundles all necessary dependencies into the application.

```rust
// Everything common to Rust programs
use agera::common::*;

// Everything common to the entity-component-system.
// Agera SDK uses `bevy_ecs` for the entity-component-system
// and adds several operation traits to Entity.
use agera::ecs::common::*;

agera::application::root(); // Entity
agera::application::root().spawn_child(()).id(); // Entity
```

## Development progress

Agera is still in development, therefore it is not released yet. It should feel similiar to Adobe AIR or Flash Platform, but using the entity-component-system pattern and containing additional built-in features.

- [Here are short plans](docs/plans.md)

Additional notes:

- Agera projects must use the nightly Rust compiler to leverage language features, such as use of declarative macros and `try` blocks.