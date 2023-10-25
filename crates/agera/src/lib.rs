/*!
The API used to write Agera rich internet applications.

# Basics

Typically Agera applications are created either through an IDE or the Agera command line.
Rust programs should typically contain the following `use` item:

```
use agera::common::*;
```

# Entities

Agera uses a hierarchical Entity model, where multiple components are attached to an Entity,
defined by the `agera::ecs` module.

# Graphical experience

The `agera::display` and `agera::ui` modules are used to display graphics and controls
to the screen. It supports in-depth settings of display objects, such as registration point and filters
and operations such as measurement of object bounds.

# Working with files

The `agera::file` module provides ways of operating on files, either by path or by reference.
It abstracts away working with files that belong to the application.

```
use agera::file::*;

// A file path
let file = File::new("app://asset.svg");

// Synchronous operation
println!("{}", file.exists());
```

# Working with events

Event conditions are commonly expressed as `EventEmitter` objects, to which the programmer
may attach listeners by using the `.listener` method.

```
use agera::common::*;

// Registering listener
let listener = button.on_click().listener(|e| {
    // Action
});

// Removing listener
listener.remove();
```
*/

#![feature(decl_macro)]
#![feature(io_error_more)]

pub mod application;
pub mod common;
pub mod ecs;
pub mod file;
pub mod target;
pub mod timer;
pub mod util;