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

    #[wasm_bindgen(method, js_name = toFileSystemReference)]
    fn to_file_system_reference(this: &JSFileReference) -> JSFileSystemReference;

    #[derive(Clone)]
    type JSDirectoryReference;

    #[wasm_bindgen(constructor)]
    fn new(handle: JsValue) -> JSDirectoryReference;

    #[wasm_bindgen(method, js_name = name)]
    fn name(this: &JSDirectoryReference) -> String;

    #[wasm_bindgen(catch, method, js_name = entries)]
    async fn entries(this: &JSDirectoryReference) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method, js_name = getDirectory)]
    async fn get_directory(this: &JSDirectoryReference, name: String, create: bool) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method, js_name = getFile)]
    async fn get_file(this: &JSDirectoryReference, name: String, create: bool) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method, js_name = deleteEmptyDirectory)]
    async fn delete_empty_directory(this: &JSDirectoryReference, name: String) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method, js_name = deleteDirectoryAll)]
    async fn delete_directory_all(this: &JSDirectoryReference, name: String) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method, js_name = deleteFile)]
    async fn delete_file(this: &JSDirectoryReference, name: String) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, js_name = toFileSystemReference)]
    fn to_file_system_reference(this: &JSDirectoryReference) -> JSFileSystemReference;
}

#[derive(Clone)]
pub struct FileSystemReference(JSFileSystemReference);

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
pub struct FileReference(JSFileReference);

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

impl From<FileReference> for FileSystemReference {
    fn from(value: FileReference) -> Self {
        FileSystemReference(value.0.to_file_system_reference())
    }
}

#[derive(Clone)]
pub struct DirectoryReference(JSDirectoryReference);

impl DirectoryReference {
    pub fn name(&self) -> String {
        self.0.name()
    }

    pub async fn entries(&self) -> io::Result<Vec<FileSystemReference>> {
        let entries = self.0.entries().await.map_err(|error| js_io_error_to_rs_io_error(error, true))?;
        let mut entries_2 = vec![];
        for entry in js_sys::Array::try_from(entries).unwrap() {
            entries_2.push(FileSystemReference(entry.try_into().unwrap()));
        }
        Ok(entries_2)
    }

    pub async fn get_directory(&self, name: &str) -> io::Result<DirectoryReference> {
        let reference = self.0.get_directory(name.into(), false).await.map_err(|error| js_io_error_to_rs_io_error(error, true))?;
        Ok(DirectoryReference(reference.try_into().unwrap()))
    }

    pub async fn get_directory_or_create(&self, name: &str) -> io::Result<DirectoryReference> {
        let reference = self.0.get_directory(name.into(), true).await.map_err(|error| js_io_error_to_rs_io_error(error, true))?;
        Ok(DirectoryReference(reference.try_into().unwrap()))
    }

    pub async fn get_file(&self, name: &str) -> io::Result<FileReference> {
        let reference = self.0.get_file(name.into(), false).await.map_err(|error| js_io_error_to_rs_io_error(error, false))?;
        Ok(FileReference(reference.try_into().unwrap()))
    }

    pub async fn get_file_or_create(&self, name: &str) -> io::Result<FileReference> {
        let reference = self.0.get_file(name.into(), true).await.map_err(|error| js_io_error_to_rs_io_error(error, false))?;
        Ok(FileReference(reference.try_into().unwrap()))
    }

    pub async fn delete_empty_directory(&self, name: &str) -> io::Result<()> {
        self.0.delete_empty_directory(name.into()).await.map(|_| ())
            .map_err(|error| js_io_error_to_rs_io_error_for_delete_directory(error))
    }

    pub async fn delete_directory_all(&self, name: &str) -> io::Result<()> {
        self.0.delete_directory_all(name.into()).await.map(|_| ())
            .map_err(|error| js_io_error_to_rs_io_error_for_delete_directory(error))
    }

    pub async fn delete_file(&self, name: &str) -> io::Result<()> {
        self.0.delete_file(name.into()).await.map(|_| ()).map_err(|error| js_io_error_to_rs_io_error(error, false))
    }
}

impl From<DirectoryReference> for FileSystemReference {
    fn from(value: DirectoryReference) -> Self {
        FileSystemReference(value.0.to_file_system_reference())
    }
}