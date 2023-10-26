use crate::common::*;
use std::io;
use wasm_bindgen::prelude::*;
use crate::file::platforms::browser::{
    js_io_error_to_rs_io_error,
    js_io_error_to_rs_io_error_for_delete_directory,
};

#[wasm_bindgen(module = "browser.js")]
extern "C" {
    #[derive(Clone)]
    type JSFileSystemReference;

    #[wasm_bindgen(constructor)]
    fn new(handle: JsValue) -> JSFileSystemReference;

    #[wasm_bindgen(method, js_name = name)]
    fn name(this: &JSFileSystemReference) -> String;

    #[wasm_bindgen(method, js_name = asDirectory)]
    fn as_directory(this: &JSFileSystemReference) -> Option<JSDirectoryReference>;

    #[wasm_bindgen(method, js_name = asFile)]
    fn as_file(this: &JSFileSystemReference) -> Option<JSFileReference>;

    #[derive(Clone)]
    type JSFileReference;

    #[wasm_bindgen(constructor)]
    fn new(handle: JsValue) -> JSFileReference;

    #[wasm_bindgen(method, js_name = name)]
    fn name(this: &JSFileReference) -> String;

    #[wasm_bindgen(catch, method, js_name = readBytes)]
    async fn read_bytes(this: &JSFileReference) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method, js_name = write)]
    async fn write(this: &JSFileReference, bytes: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method, js_name = modificationEpochMilliseconds)]
    async fn modification_epoch_milliseconds(this: &JSFileReference) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method, js_name = size)]
    async fn size(this: &JSFileReference) -> Result<JsValue, JsValue>;

    #[derive(Clone)]
    type JSDirectoryReference;

    #[wasm_bindgen(constructor)]
    fn new(handle: JsValue) -> JSDirectoryReference;

    #[wasm_bindgen(method, js_name = name)]
    fn name(this: &JSDirectoryReference) -> String;

    #[wasm_bindgen(catch, method, js_name = entries)]
    fn entries(this: &JSDirectoryReference) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method, js_name = getDirectory)]
    fn get_directory(this: &JSDirectoryReference, name: String, create: bool) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method, js_name = getFile)]
    fn get_file(this: &JSDirectoryReference, name: String, create: bool) -> Result<JsValue, JsValue>;


}

#[derive(Clone)]
pub struct FileSystemReference(pub JSFileSystemReference);

impl FileSystemReference {
    pub fn name(&self) -> String {
        self.0.name()
    }

    pub fn as_directory(&self) -> Option<DirectoryReference> {
        self.0.as_directory().map(|d| DirectoryReference(d))
    }

    pub fn as_file(&self) -> Option<FileReference> {
        self.0.as_file().map(|file| FileReference(file))
    }
}

#[derive(Clone)]
pub struct FileReference(pub JSFileReference);

impl FileReference {
    pub async fn read_bytes(&self) -> io::Result<Bytes> {
        self.0.read_bytes().await.map(|ba| Bytes::from(js_sys::Uint8Array::try_from(ba).unwrap().to_vec())).map_err(|error| js_io_error_to_rs_io_error(error, false))
    }

    pub async fn read_utf8(&self) -> io::Result<String> {
        Ok(String::from_utf8_lossy(&self.read_bytes().await?).into_owned())
    }

    pub async fn write(&self, data: &[u8]) -> io::Result<()> {
        let uint8array = js_sys::Uint8Array::from(data);
        self.0.write(uint8array.buffer().into()).await.map(|_| ()).map_err(|error| js_io_error_to_rs_io_error(error, false))
    }

    pub async fn modification_date(&self) -> io::Result<std::time::SystemTime> {
        let ms = self.0.modification_epoch_milliseconds().await.map_err(|error| js_io_error_to_rs_io_error(error, false))?;
        let ms: u64 = unsafe { ms.as_f64().unwrap().to_int_unchecked() };
        Ok(std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_millis(ms))
    }

    pub fn name(&self) -> String {
        self.0.name()
    }

    pub async fn size(&self) -> io::Result<usize> {
        let size = self.0.size().await.map_err(|error| js_io_error_to_rs_io_error(error, false))?;
        Ok(unsafe { size.as_f64().unwrap().to_int_unchecked() })
    }
}