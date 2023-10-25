# Progress track

## Application build

- Refer to https://github.com/rust-mobile/xbuild

## CLI

- Build, run, export
  - Only Windows/OSX/Linux/Android/Web are supported currently.

## Processing

- [x] Setup application target details

## agera::file

File API.

- [x] Access to various directories, including user directories
- [ ] For the browser, in `bootstrap.rs`, embed all installation files by writing them using `File` from the crate; in this case, the files are copied to a directory near `bootstrap.rs` and included by `bootstrap.rs` using `include_bytes!`.
- [ ] File streaming read and write (not a priority for now)

Support `FileReference`, an alternative to `File` that complies with the Web File API, providing only asynchronous operations, used by file pickers more generally.

- [ ] `FileReference`
- [ ] File picker functions

## agera::display

- Refer to https://wgpu.rs

How you work with display objects:

```rust
use agera::display::*;

// `DisplayObject` is an entity with additional methods
let object: DisplayObject = DisplayObject::new();
object.has::<TComponent>();
object.set_alpha(0);
object.add_child(&child);

let entity: Entity = object.into();;

// Entity back into a DisplayObject
let object = DisplayObject::try_from(entity.as_entity());
```

Types for display objects are built by implementing `DisplayObjectDelegate`, thus inheriting necessary methods to quickly work with display objects.

```rust
use agera::common::*;
use agera::display::*;

struct CustomDisplayObject(Entity);

impl CustomDisplayObject {
    fn new() {
        Self(DisplayObject::new().into())
    }
}

impl DisplayObjectDelegate for CustomDisplayObject {
    fn delegate(&self) -> Entity {
        self.0
    }
}
```

## agera::ui

Agera controls, `AgeraControl`, similiarly to display objects, use a delegate trait, `AgeraControlDelegate`. Agera controls are display objects under the hood.