/*!
The foundational APIs used to write Agera rich internet applications.

# The Basics

Agera applications are created either through the Agera command line interface or
an integrated development environment. Rust programs should include the following `use` item:

```
use agera::common::*;
```

# Nodes

Agera uses a Node graph using the [`util::inheritance`] module.
You can define your own classes that extend other Node classes.

# Graphical Experience

The `agera::display` and `agera::controls` modules are used to display graphics and controls
to the screen. Agera supports in-depth settings of display objects, such as registration point and filters
and operations such as measurement of object bounds.

# Working with Files

The `agera::file` module provides ways of operating on files, either by path or by reference.
It abstracts away working with files that belong to the application.

```
use agera::file::*;

// A file path
let file = File::new("app://asset.svg");

// Synchronous operation
println!("{}", file.exists());

// Asynchronous operation
println!("{}", file.exists_async().await);
```

# Working with Events

Event conditions are commonly expressed as `EventEmitter` objects, to which the programmer
may attach listeners by using the `.listener` method.

```
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
#![feature(structural_match)]
#![feature(try_blocks)]

pub mod application;
pub mod common;
pub mod display;
pub mod events;
pub mod file;
pub mod geom;
pub mod platforms;
pub mod text;
pub mod timer;
pub mod util;