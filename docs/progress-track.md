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
- [ ] File streaming read and write (not a priority for now)

Support `FileReference` and `DirectoryReference`, alternatives to `File` that comply with the Web File System API, providing only asynchronous operations, used by file pickers more generally.

- [ ] `FileReference`
- [ ] `DirectoryReference`
- [ ] File picker functions

Should be properly played with:

- [ ] Use of `File` on the web browser: installation files and storage files.

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