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

- [ ] Access to various directories, including user directories
  - [ ] Windows, Linux, OSX
    - Refer to https://crates.io/crates/dirs
  - [ ] Android
  - [ ] Browser (return `None`)
- [ ] `file.exists()`
  - [ ] Native
  - [ ] Browser
- [ ] `file.is_directory()`
  - [ ] Native
  - [ ] Browser
- [ ] `file.is_file()`
  - [ ] Native
  - [ ] Browser
- [ ] `file.is_symbolic_link()`
  - [ ] Native
  - [ ] Browser
- [ ] `file.canonicalize()`
  - [ ] Native
  - [ ] Browser
- [ ] `file.canonicalize_async()`
  - [ ] Native
  - [ ] Browser
- [ ] `file.copy_to()`
  - [ ] Native
  - [ ] Browser
- [ ] `file.copy_to_async()`
  - [ ] Native
  - [ ] Browser
- [ ] `file.create_directory()`
  - [ ] Native
  - [ ] Browser
- [ ] `file.create_directory_async()`
  - [ ] Native
  - [ ] Browser
- [ ] `file.create_directory_all()`
  - [ ] Native
  - [ ] Browser
- [ ] `file.create_directory_all_async()`
  - [ ] Native
  - [ ] Browser
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
  - Guarantee that returned `File`s conform to the same scheme. In native targets, for `app:` and `app-storage:`, *do not* resolve symbolic links and relativize the physical path of the scheme to the item path.
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

Types for display objects are built with the `DisplayObject` derive macro, which provides several methods specific to `DisplayObject`:

```rust
use agera::display::*;

#[derive(DisplayObject)]
struct CustomDisplayObject {}
```