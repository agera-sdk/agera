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

    #[wasm_bindgen(catch, js_name = directoryListingAsync)]
    async fn js_directory_listing_async(path: String) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_name = deleteEmptyDirectoryAsync)]
    async fn js_delete_empty_directory_async(parent_path: String, name: String) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_name = deleteDirectoryAllAsync)]
    async fn js_delete_directory_all_async(parent_path: String, name: String) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_name = deleteFileAsync)]
    async fn js_delete_file_async(parent_path: String, name: String) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_name = writeAsync)]
    async fn js_write_async(path: String, data: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_name = modificationEpochMillisecondsAsync)]
    async fn js_modification_epoch_milliseconds_async(path: String) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_name = sizeAsync)]
    async fn js_size_async(path: String) -> Result<JsValue, JsValue>;
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

pub async fn directory_listing_async(path: String) -> io::Result<Vec<String>> {
    let listing1 = js_directory_listing_async(path).await.map_err(|error| js_io_error_to_rs_io_error(error, true))?;
    let mut listing2 = vec![];
    for name in js_sys::Array::try_from(listing1).unwrap() {
        listing2.push(name.as_string().unwrap());
    }
    Ok(listing2)
}

pub async fn delete_empty_directory_async(parent_path: String, name: String) -> io::Result<()> {
    js_delete_empty_directory_async(parent_path, name).await.map(|_| ()).map_err(|error| js_io_error_to_rs_io_error_for_delete_directory(error))
}

pub async fn delete_directory_all_async(parent_path: String, name: String) -> io::Result<()> {
    js_delete_directory_all_async(parent_path, name).await.map(|_| ()).map_err(|error| js_io_error_to_rs_io_error_for_delete_directory(error))
}

pub async fn delete_file_async(parent_path: String, name: String) -> io::Result<()> {
    js_delete_file_async(parent_path, name).await.map(|_| ()).map_err(|error| js_io_error_to_rs_io_error(error, false))
}

pub async fn write_async(path: String, data: &[u8]) -> io::Result<()> {
    let uint8array = js_sys::Uint8Array::from(data);
    js_write_async(path, uint8array.buffer().into()).await.map(|_| ()).map_err(|error| js_io_error_to_rs_io_error(error, false))
}

pub async fn modification_date_async(path: String) -> io::Result<Option<std::time::SystemTime>> {
    let ms = js_modification_epoch_milliseconds_async(path).await.map_err(|error| js_io_error_to_rs_io_error(error, false))?;
    if ms.is_undefined() {
        return Ok(None);
    }
    let ms: u64 = unsafe { ms.as_f64().unwrap().to_int_unchecked() };
    Ok(Some(std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_millis(ms)))
}

pub async fn size_async(path: String) -> io::Result<usize> {
    let size = js_size_async(path).await.map_err(|error| js_io_error_to_rs_io_error(error, false))?;
    Ok(unsafe { size.as_f64().unwrap().to_int_unchecked() })
}

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