use crate::common::*;
use std::io;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "browser.js")]
extern "C" {
    #[wasm_bindgen(js_name = existsAsync)]
    async fn js_exists_async(path: String) -> JsValue;

    #[wasm_bindgen(js_name = isDirectoryAsync)]
    async fn js_is_directory_async(path: String) -> JsValue;

    #[wasm_bindgen(js_name = isFileAsync)]
    async fn js_is_file_async(path: String) -> JsValue;

    #[wasm_bindgen(catch, js_name = createDirectoryAsync)]
    async fn js_create_directory(parent_path: String, name: String) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_name = createDirectoryAllAsync)]
    async fn js_create_directory_all(path: String) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_name = readBytesAsync)]
    async fn js_read_bytes_async(path: String) -> Result<JsValue, JsValue>;
}

pub fn within_application_directory(path: &str) -> String {
    format!("/install{path}")
}

pub fn within_application_storage_directory(path: &str) -> String {
    format!("/storage{path}")
}

pub async fn exists_async(path: String) -> bool {
    js_exists_async(path).await.as_bool().unwrap()
}

pub async fn is_directory_async(path: String) -> bool {
    js_is_directory_async(path).await.as_bool().unwrap()
}

pub async fn is_file_async(path: String) -> bool {
    js_is_file_async(path).await.as_bool().unwrap()
}

pub async fn create_directory_async(parent_path: String, name: String) -> io::Result<()> {
    js_create_directory(parent_path, name).await.map(|_| ()).map_err(|error| js_io_error_to_rs_io_error(error, true))
}

pub async fn create_directory_all_async(path: String) -> io::Result<()> {
    js_create_directory_all(path).await.map(|_| ()).map_err(|error| js_io_error_to_rs_io_error(error, true))
}

pub async fn read_bytes_async(path: String) -> io::Result<Bytes> {
    js_read_bytes_async(path).await.map(|ba| Bytes::from(js_sys::Uint8Array::try_from(ba).unwrap().to_vec())).map_err(|error| js_io_error_to_rs_io_error(error, false))
}

pub async fn read_utf8_async(path: String) -> io::Result<String> {
    js_read_bytes_async(path).await.map(|ba| String::from_utf8_lossy(&js_sys::Uint8Array::try_from(ba).unwrap().to_vec()).into_owned()).map_err(|error| js_io_error_to_rs_io_error(error, false))
}

fn js_io_error_to_rs_io_error(error: JsValue, is_directory: bool) -> io::Error {
    match error.as_f64().unwrap() {
        0.0 => io::Error::new(io::ErrorKind::NotFound, "File or directory not found"),
        1.0 => if is_directory {
            io::Error::new(io::ErrorKind::NotADirectory, "Not a directory")
        } else {
            io::Error::new(io::ErrorKind::IsADirectory, "Found directory")
        },
        2.0 => io::Error::new(io::ErrorKind::PermissionDenied, "Permission denied"),
        3.0 => io::Error::new(io::ErrorKind::InvalidFilename, "Invalid filename"),
        4.0 |
        5.0 => io::Error::new(io::ErrorKind::Other, "Invalidated origin private file system state"),
        _ => io::Error::new(io::ErrorKind::Other, "Unknown error"),
    }
}