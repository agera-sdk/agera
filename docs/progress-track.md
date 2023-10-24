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
- [ ] `file.exists()`
  - [ ] Native
  - [x] Browser
- [ ] `file.exists_async()`
  - [ ] Native
  - [x] Browser
- [ ] `file.is_directory()`
  - [ ] Native
  - [x] Browser
- [ ] `file.is_directory_async()`
  - [ ] Native
  - [x] Browser
- [ ] `file.is_file()`
  - [ ] Native
  - [x] Browser
- [ ] `file.is_file_async()`
  - [ ] Native
  - [x] Browser
- [ ] `file.is_symbolic_link()`
  - [ ] Native
  - [x] Browser
- [ ] `file.is_symbolic_link_async()`
  - [ ] Native
  - [x] Browser
- [ ] `file.canonicalize()`
  - [ ] Native
  - [x] Browser
- [ ] `file.canonicalize_async()`
  - [ ] Native
  - [x] Browser
- [ ] `file.copy_to()`
  - [ ] Native
  - [x] Browser
- [ ] `file.copy_to_async()`
  - [ ] Native
  - [x] Browser
- [ ] `file.create_directory()`
  - [ ] Native
  - [x] Browser
- [ ] `file.create_directory_async()`
  - [ ] Native
  - [x] Browser
- [ ] `file.create_directory_all()`
  - [ ] Native
  - [x] Browser
- [ ] `file.create_directory_all_async()`
  - [ ] Native
  - [x] Browser
- [ ] `file.read_bytes()`
  - [ ] Native
  - [ ] Browser
- [ ] `file.read_bytes_async()`
  - [ ] Native
  - [ ] Browser
- [ ] `file.read_utf8()`
  - [ ] Native
  - [ ] Browser
- [ ] `file.read_utf8_async()`
  - [ ] Native
  - [ ] Browser
- [ ] `file.directory_listing()`
  - Guarantee that returned `File`s conform to the same scheme. In native targets, for `app:` and `app-storage:`, *do not* resolve symbolic links and relativize the physical path of the scheme to the directory path and add the item's name.
  - [ ] Native
  - [ ] Browser
- [ ] `file.directory_listing_async()`
  - Guarantee that returned `File`s conform to the same scheme, similiar to `directory_listing()`
  - [ ] Native
  - [ ] Browser
- [ ] `file.delete_directory_if_empty()`
  - [ ] Native
  - [ ] Browser
- [ ] `file.delete_directory_if_empty_async()`
  - [ ] Native
  - [ ] Browser
- [ ] `file.delete_directory_all()`
  - [ ] Native
  - [ ] Browser
- [ ] `file.delete_directory_all_async()`
  - [ ] Native
  - [ ] Browser
- [ ] `file.delete_file()`
  - [ ] Native
  - [ ] Browser
- [ ] `file.delete_file_async()`
  - [ ] Native
  - [ ] Browser
- [ ] `file.rename()`
  - [ ] Native
  - [ ] Browser
- [ ] `file.rename_async()`
  - [ ] Native
  - [ ] Browser
- [ ] `file.write()`
  - [ ] Native
  - [ ] Browser
- [ ] `file.write_async()`
  - [ ] Native
  - [ ] Browser
- [ ] `file.creation_date()`
  - [ ] Native
  - [ ] Browser
- [ ] `file.creation_date_async()`
  - [ ] Native
  - [ ] Browser
- [ ] `file.modification_date()`
  - [ ] Native
  - [ ] Browser
- [ ] `file.modification_date_async()`
  - [ ] Native
  - [ ] Browser
- [ ] `file.size()`
  - [ ] Native
  - [ ] Browser
- [ ] `file.size_async()`
  - [ ] Native
  - [ ] Browser
- [ ] File streaming read and write for supported platforms

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