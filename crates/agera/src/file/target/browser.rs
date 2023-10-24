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

pub async fn create_directory(parent_path: String, name: String) -> io::Result<()> {
    js_create_directory(parent_path, name).await.map(|_| ()).map_err(|error| js_io_error_to_rs_io_error(error, true))
}

pub async fn create_directory_all(path: String) -> io::Result<()> {
    js_create_directory_all(path).await.map(|_| ()).map_err(|error| js_io_error_to_rs_io_error(error, true))
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
        _ => io::Error::new(io::ErrorKind::Other, "Unknown error"),
    }
}