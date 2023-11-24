# Inheritance

Agera supports structure inheritance over the Rust language for use with the `Entity` type. 

Differences from existing object-oriented programming inheritance in languages like Java:

* Method overriding does not exist; subtype methods shadow super type's methods.

To define an entity type, use `entity_type!`, specifying all of the inherited types correctly, in descending order, by using a less-than (`<`) delimited sequence.

The following is an explanatory `DisplayObject` subtype:

```rust
use agera::{display::*, entity::*};

entity_type! {
    // Visibility, attributes and RustDoc comments can be applied
    // to this struct.
    struct S: DisplayObject < Entity {
        // Fields described inside the `struct` definition
        // are mapped to getter and setter functions,
        // `field_name()` and `set_field_name()` respectively.
        // Fields always have a default value.
        //
        // A field is cloned when read, since it may not implement the Copy trait.
        a: f64 = 0.0,

        // A field may specify visibility.
        pub c: f64 = 0.0,

        /// RustDoc comments are supported, as well as attributes.
        #[cfg(hidden)]
        d: f64 = 0.0,

        // `ref` indicates a mutable reference field of type `Arc<T>`.
        ref e: f64 = 0.0,
    }

    // Constructor function. It may specify attributes, visibility, parameters and
    // generics.
    //
    // The constructor may be invoked through the `S::new(...)` method.
    //
    pub fn constructor() {
        super();
        // `this` is available here.
    }
}
```

## Base type as parameter

Receiving a specific base Entity type can be done by taking a parameter of type `impl AsRef<T>`, where `T` is the entity type:

```rust
fn f(entity: impl AsRef<Entity>) {
    let entity: Entity = entity.as_ref();
    // Action
}
```

This allows directly passing subtypes of `Entity` as parameters.

## Type relationship

* Use the `is::<T>()` method of an `Entity` to match for a subtype.
* Use the `to::<T>()` method of an `Entity` to convert to a subtype.