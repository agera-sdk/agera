/*!
The API used to write Agera rich internet applications.

## Basics

Typically Agera applications are created either through an IDE or the Agera command line.
Rust programs should typically contain the following `use` item:

```
use agera::common::*;
```

## Entities

Agera uses a hierarchical Entity model, where multiple components are attached to an Entity.

## Graphical experience

The `agera::display` and `agera::ui` modules are used to display graphics and controls
to the screen. It supports in-depth settings of display objects, such as registration point and filters
and operations such as measurement of object bounds.
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