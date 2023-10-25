use crate::common::*;
use std::io;
use wasm_bindgen::prelude::*;

fn js_io_error_to_rs_io_error(error: JsValue, is_directory: bool) -> io::Error {
    let error = error.as_f64().unwrap();
    if error == 0.0 {
        io::Error::new(io::ErrorKind::NotFound, "File or directory not found")
    } else if error == 1.0 {
        if is_directory {
            io::Error::new(io::ErrorKind::NotADirectory, "Not a directory")
        } else {
            io::Error::new(io::ErrorKind::IsADirectory, "Found directory")
        }
    } else if error == 2.0 {
        io::Error::new(io::ErrorKind::PermissionDenied, "Permission denied")
    } else if error == 3.0 {
        io::Error::new(io::ErrorKind::InvalidFilename, "Invalid filename")
    } else if [4.0, 5.0].contains(&error) {
        io::Error::new(io::ErrorKind::Other, "Invalidated origin private file system state")
    } else {
        io::Error::new(io::ErrorKind::Other, "Unknown error")
    }
}

fn js_io_error_to_rs_io_error_for_delete_directory(js_error: JsValue) -> io::Error {
    let error = js_error.as_f64().unwrap();
    if error == 6.0 {
        io::Error::new(io::ErrorKind::DirectoryNotEmpty, "Directory not empty")
    } else {
        js_io_error_to_rs_io_error(js_error, true)
    }
}