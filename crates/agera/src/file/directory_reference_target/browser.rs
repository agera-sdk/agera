use crate::common::*;
use std::io;
use wasm_bindgen::prelude::*;
use crate::file::target::browser::{
    js_io_error_to_rs_io_error,
    js_io_error_to_rs_io_error_for_delete_directory,
};