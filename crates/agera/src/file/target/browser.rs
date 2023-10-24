use std::io;
use wasm_bindgen::prelude::*;
use js_sys::*;

#[wasm_bindgen(module = "browser.js")]
extern "C" {
    #[wasm_bindgen(js_name = existsAsync)]
    async fn js_exists_async(path: String) -> JsValue;

    #[wasm_bindgen(js_name = isDirectoryAsync)]
    async fn js_is_directory_async(path: String) -> JsValue;

    #[wasm_bindgen(js_name = isFileAsync)]
    async fn js_is_file_async(path: String) -> JsValue;
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