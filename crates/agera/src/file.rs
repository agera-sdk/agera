/*!
File API.
*/

use crate::{common::*, target::{if_native_target, if_browser_target}};
use file_paths::*;
use std::path::Path;

pub(crate) mod target;

/// Represents a file, either in the file system, application or
/// application storage directory.
/// 
/// The following URIs are supported:
/// 
/// * `file:` — A file located in the regular file system.
/// * `app:` — A file located in the application installation directory.
/// * `app-storage:` — A file located in the application storage directory.
/// 
/// # Browser support
/// 
/// * Synchronous operations are supported on all platforms except for the browser.
/// Synchronous operations are expected to panic when running in the browser.
/// * The `file:` scheme is not supported in the browser. If it is required
/// for the application to pick user files or directories, consider using
/// file pickers and thus `FileReference`.
///
#[derive(Clone, PartialEq)]
pub struct File {
    scheme: FileScheme,
    path: String,
}

impl File {
    /// Creates a file with a specified native path or URI.
    /// `path_or_uri` is treated as an URI if it starts with either
    /// `file:`, `app:` or `app-storage:`.
    pub fn new(path_or_uri: &str) -> File {
        if path_or_uri.starts_with("file:") {
            File {
                scheme: FileScheme::File,
                path: FlexPath::new_native(&uri_to_native_path(path_or_uri)).to_string(),
            }
        } else if path_or_uri.starts_with("app:") {
            let path = regex_replace!(r"^/{0,2}", &decode_uri(&path_or_uri[4..]), |_| "/".to_owned()).into_owned();
            File {
                scheme: FileScheme::App,
                path: FlexPath::new_common(&path).to_string(),
            }
        } else if path_or_uri.starts_with("app-storage:") {
            let path = regex_replace!(r"^/{0,2}", &decode_uri(&path_or_uri[12..]), |_| "/".to_owned()).into_owned();
            File {
                scheme: FileScheme::AppStorage,
                path: FlexPath::new_common(&path).to_string(),
            }
        } else {
            File {
                scheme: FileScheme::File,
                path: FlexPath::new_native(path_or_uri).to_string(),
            }
        }
    }

    /// The application's installation directory. The result of this function is equivalent
    /// to `File::new("app://")`.
    pub fn application_directory() -> File {
        File::new("app://")
    }

    /// The application's storage directory. The result of this function is equivalent
    /// to `File::new("app-storage://")`.
    pub fn application_storage_directory() -> File {
        File::new("app-storage://")
    }

    /// The user's downloads directory.
    pub fn downloads_directory() -> Option<File> {
        Some(File {
            scheme: FileScheme::File,
            path: downloads_directory()?,
        })
    }

    /// The user's documents directory.
    pub fn documents_directory() -> Option<File> {
        Some(File {
            scheme: FileScheme::File,
            path: documents_directory()?,
        })
    }

    /// The user's pictures directory.
    pub fn pictures_directory() -> Option<File> {
        Some(File {
            scheme: FileScheme::File,
            path: pictures_directory()?,
        })
    }

    /// The user's music directory.
    pub fn music_directory() -> Option<File> {
        Some(File {
            scheme: FileScheme::File,
            path: music_directory()?,
        })
    }

    /// The user's videos directory.
    pub fn videos_directory() -> Option<File> {
        Some(File {
            scheme: FileScheme::File,
            path: videos_directory()?,
        })
    }

    /// Returns the native path of the file, if it has the scheme `file:`.
    pub fn native_path(&self) -> Option<String> {
        if self.scheme == FileScheme::File { Some(self.path.clone()) } else { None }
    }

    /// Returns the URI of the file.
    pub fn uri(&self) -> String {
        match self.scheme {
            FileScheme::File => {
                native_path_to_uri(&self.path)
            },
            FileScheme::App => {
                format!("app:/{}", encode_uri(&self.path))
            },
            FileScheme::AppStorage => {
                format!("app-storage:/{}", encode_uri(&self.path))
            },
        }
    }

    /// Resolves path to a file or directory.
    pub fn resolve_path(&self, path: &str) -> File {
        File {
            scheme: self.scheme,
            path: self.flex_path().resolve(path).to_string(),
        }
    }

    /// Returns a flexible representation of the `File`'s path,
    /// which provides a number of operations.
    pub fn flex_path(&self) -> FlexPath {
        FlexPath::new(&self.path, self.flex_path_variant())
    }

    fn flex_path_variant(&self) -> FlexPathVariant {
        match self.scheme {
            FileScheme::File => FlexPathVariant::native(),
            _ => FlexPathVariant::Common,
        }
    }

    /// Returns the name of the file or directory.
    pub fn name(&self) -> String {
        self.flex_path().base_name()
    }

    /// Indicates the extension of the file, including the first
    /// dot character (`.`).
    pub fn extension(&self) -> Option<String> {
        let c = regex_captures!(r"\..+$", &self.path);
        c.map(|c| c.to_owned())
    }

    /// Returns the parent directory of the file or directory, or
    /// the same `File` if it has no parent directory.
    pub fn parent(&self) -> File {
        self.resolve_path("..")
    }

    fn path_omega(&self) -> String {
        let mut p = self.path.clone();
        match self.scheme {
            FileScheme::App => {
                p = format!("{}{p}", application_directory());
            },
            FileScheme::AppStorage => {
                p = format!("{}{p}", application_storage_directory());
            },
            FileScheme::File => {},
        }
        FlexPath::new(&p, self.flex_path_variant()).to_string_with_flex_separator()
    }

    /// Indicates whether a file or directory exists, synchronously.
    pub fn exists(&self) -> bool {
        if_native_target! {{
            Path::new(&self.path_omega()).exists()
        }}
        if_browser_target! {{
            unsupported_browser_sync_operation!();
        }}
    }

    /// Indicates whether a file or directory exists, asynchronously.
    pub async fn exists_async(&self) -> bool {
        if_native_target! {{
            tokio::fs::metadata(&self.path_omega()).await.is_ok()
        }}
        if_browser_target! {{
            target::browser::exists_async(self.path_omega()).await
        }}
    }

    /// Indicates whether the `File` object is a directory, synchronously.
    pub fn is_directory(&self) -> bool {
        if_native_target! {{
            std::fs::metadata(&self.path_omega()).map(|data| data.is_dir()).unwrap_or(false)
        }}
        if_browser_target! {{
            unsupported_browser_sync_operation!();
        }}
    }

    /// Indicates whether the `File` object is a directory, asynchronously.
    pub async fn is_directory_async(&self) -> bool {
        if_native_target! {{
            tokio::fs::metadata(&self.path_omega()).await.map(|data| data.is_dir()).unwrap_or(false)
        }}
        if_browser_target! {{
            target::browser::is_directory_async(self.path_omega()).await
        }}
    }

    /// Indicates whether the `File` object is a file, synchronously.
    pub fn is_file(&self) -> bool {
        if_native_target! {{
            std::fs::metadata(&self.path_omega()).map(|data| data.is_file()).unwrap_or(false)
        }}
        if_browser_target! {{
            unsupported_browser_sync_operation!();
        }}
    }

    /// Indicates whether the `File` object is a file, asynchronously.
    pub async fn is_file_async(&self) -> bool {
        if_native_target! {{
            tokio::fs::metadata(&self.path_omega()).await.map(|data| data.is_file()).unwrap_or(false)
        }}
        if_browser_target! {{
            target::browser::is_file_async(self.path_omega()).await
        }}
    }

    /// Indicates whether the `File` object is a symbolic link, synchronously.
    pub fn is_symbolic_link(&self) -> bool {
        if_native_target! {{
            std::fs::metadata(&self.path_omega()).map(|data| data.is_symlink()).unwrap_or(false)
        }}
        if_browser_target! {{
            unsupported_browser_sync_operation!();
        }}
    }

    /// Indicates whether the `File` object is a symbolic link, asynchronously.
    pub async fn is_symbolic_link_async(&self) -> bool {
        if_native_target! {{
            tokio::fs::metadata(&self.path_omega()).await.map(|data| data.is_symlink()).unwrap_or(false)
        }}
        if_browser_target! {{
            false
        }}
    }

    /// Canonicalizes the file path, synchronously.
    /// For non `file:` schemes, this returns the same path.
    ///
    pub fn canonicalize(&self) -> File {
        if_native_target! {{
            if self.scheme != FileScheme::File {
                return self.clone();
            }
            if let Some(result) = Path::new(&self.path_omega()).canonicalize().ok().and_then(|result| result.to_str()) {
                return File { scheme: FileScheme::File, path: result.to_owned() };
            }
            return self.clone();
        }}
        if_browser_target! {{
            unsupported_browser_sync_operation!();
        }}
    }

    /// Canonicalizes the file path, asynchronously.
    /// For non `file:` schemes, this returns the same path.
    pub async fn canonicalize_async(&self) -> File {
        if_native_target! {{
            if self.scheme != FileScheme::File {
                return self.clone();
            }
            if let Some(result) = tokio::fs::canonicalize(&self.path_omega()).await.ok().map(|result| result.to_string_lossy().into_owned()) {
                return File { scheme: FileScheme::File, path: result };
            }
            return self.clone();
        }}
        if_browser_target! {{
            self.clone()
        }}
    }

    /// Copies a file or directory to another path specified by
    /// `location`, overriding any contents at `location`.
    /// This is a synchronous operation.
    ///
    pub fn copy_to(&self, location: &File) -> std::io::Result<()> {
        if_native_target! {{
            must_write_here_yet;
        }}
        if_browser_target! {{
            let _ = location;
            unsupported_browser_sync_operation!();
        }}
    }

    /// Copies a file or directory to another path specified by
    /// `location`, overriding any contents at `location`.
    /// This is an asynchronous operation.
    /// 
    /// # Browser support
    /// 
    /// This operation is currently not supported in the browser
    /// and thus should panic.
    ///
    pub async fn copy_to_async(&self, location: &File) -> std::io::Result<()> {
        if_native_target! {{
            must_write_here_yet;
        }}
        if_browser_target! {{
            let _ = location;
            unsupported_browser_operation!();
        }}
    }

    /// Creates an empty directory synchronously.
    pub fn create_directory(&self) -> std::io::Result<()> {
        if_native_target! {{
            must_write_here_yet;
        }}
        if_browser_target! {{
            unsupported_browser_sync_operation!();
        }}
    }

    /// Creates an empty directory asynchronously.
    pub async fn create_directory_async(&self) -> std::io::Result<()> {
        if_native_target! {{
            must_write_here_yet;
        }}
        if_browser_target! {{
            target::browser::create_directory_async(self.parent().path_omega(), self.flex_path().base_name()).await
        }}
    }

    /// Creates a directory and its parent directories synchronously.
    pub fn create_directory_all(&self) -> std::io::Result<()> {
        if_native_target! {{
            must_write_here_yet;
        }}
        if_browser_target! {{
            unsupported_browser_sync_operation!();
        }}
    }

    /// Creates a directory and its parent directories asynchronously.
    pub async fn create_directory_all_async(&self) -> std::io::Result<()> {
        if_native_target! {{
            must_write_here_yet;
        }}
        if_browser_target! {{
            target::browser::create_directory_all_async(self.path_omega()).await
        }}
    }

    /// Reads the bytes from a file synchronously.
    pub fn read_bytes(&self) -> std::io::Result<Bytes> {
        if_native_target! {{
            must_write_here_yet;
        }}
        if_browser_target! {{
            unsupported_browser_sync_operation!();
        }}
    }

    /// Reads the bytes from a file asynchronously.
    pub async fn read_bytes_async(&self) -> std::io::Result<Bytes> {
        if_native_target! {{
            must_write_here_yet;
        }}
        if_browser_target! {{
            target::browser::read_bytes_async(self.path_omega()).await
        }}
    }

    /// Reads an UTF-8 string from a file synchronously.
    pub fn read_utf8(&self) -> std::io::Result<Bytes> {
        if_native_target! {{
            must_write_here_yet;
        }}
        if_browser_target! {{
            unsupported_browser_sync_operation!();
        }}
    }

    /// Reads an UTF-8 string from a file asynchronously.
    pub async fn read_utf8_async(&self) -> std::io::Result<String> {
        if_native_target! {{
            must_write_here_yet;
        }}
        if_browser_target! {{
            target::browser::read_utf8_async(self.path_omega()).await
        }}
    }

    /// Returns the listing of directories and files in a directory,
    /// synchronously.
    pub fn directory_listing(&self) -> std::io::Result<File> {
        if_native_target! {{
            must_write_here_yet;
        }}
        if_browser_target! {{
            unsupported_browser_sync_operation!();
        }}
    }

    /// Returns the listing of directories and files in a directory,
    /// asynchronously.
    pub async fn directory_listing_async(&self) -> std::io::Result<Vec<File>> {
        if_native_target! {{
            must_write_here_yet;
        }}
        if_browser_target! {{
            let listing_1 = target::browser::directory_listing_async(self.path_omega()).await?;
            let mut listing_2 = vec![];
            for name in listing_1 {
                listing_2.push(self.resolve_path(&name));
            }
            Ok(listing_2)
        }}
    }

    /// Deletes an empty directory synchronously.
    pub fn delete_empty_directory(&self) -> std::io::Result<()> {
        if_native_target! {{
            must_write_here_yet;
        }}
        if_browser_target! {{
            unsupported_browser_sync_operation!();
        }}
    }

    /// Deletes an empty directory asynchronously.
    pub async fn delete_empty_directory_async(&self) -> std::io::Result<()> {
        if_native_target! {{
            must_write_here_yet;
        }}
        if_browser_target! {{
            target::browser::delete_empty_directory_async(self.parent().path_omega(), self.flex_path().base_name()).await
        }}
    }

    /// Deletes a directory recursively synchronously.
    pub fn delete_directory_all(&self) -> std::io::Result<()> {
        if_native_target! {{
            must_write_here_yet;
        }}
        if_browser_target! {{
            unsupported_browser_sync_operation!();
        }}
    }

    /// Deletes a directory recursively asynchronously.
    pub async fn delete_directory_all_async(&self) -> std::io::Result<()> {
        if_native_target! {{
            must_write_here_yet;
        }}
        if_browser_target! {{
            target::browser::delete_directory_all_async(self.parent().path_omega(), self.flex_path().base_name()).await
        }}
    }

    /// Deletes a file synchronously.
    pub fn delete_file(&self) -> std::io::Result<()> {
        if_native_target! {{
            must_write_here_yet;
        }}
        if_browser_target! {{
            unsupported_browser_sync_operation!();
        }}
    }

    /// Deletes a file asynchronously.
    pub async fn delete_file_async(&self) -> std::io::Result<()> {
        if_native_target! {{
            must_write_here_yet;
        }}
        if_browser_target! {{
            target::browser::delete_file_async(self.parent().path_omega(), self.flex_path().base_name()).await
        }}
    }

    /// Moves a file or directory to a new path `to_path`, synchronously. This method overrides
    /// any file contents present at the path `to_path`.
    pub fn move_to(&self, to_path: &File) -> std::io::Result<()> {
        if_native_target! {{
            must_write_here_yet;
        }}
        if_browser_target! {{
            let _ = to_path;
            unsupported_browser_sync_operation!();
        }}
    }

    /// Moves a file or directory to a new path `to_path`, asynchronously. This method overrides
    /// any file contents present at the path `to_path`.
    /// 
    /// # Browser support
    /// 
    /// This operation is currently not supported in the browser
    /// and thus should panic.
    ///
    pub async fn move_to_async(&self, to_path: &File) -> std::io::Result<()> {
        if_native_target! {{
            must_write_here_yet;
        }}
        if_browser_target! {{
            let _ = to_path;
            unsupported_browser_operation!();
        }}
    }

    /// Writes data to a file synchronously.
    pub fn write<T: AsRef<[u8]>>(&self, data: T) -> std::io::Result<()> {
        if_native_target! {{
            must_write_here_yet;
        }}
        if_browser_target! {{
            let _ = data;
            unsupported_browser_sync_operation!();
        }}
    }

    /// Writes data to a file asynchronously.
    pub async fn write_async<T: AsRef<[u8]>>(&self, data: T) -> std::io::Result<()> {
        if_native_target! {{
            must_write_here_yet;
        }}
        if_browser_target! {{
            target::browser::write_async(self.path_omega(), data.as_ref()).await
        }}
    }

    /// The creation date of a file or directory. This method returns synchronously.
    pub fn creation_date(&self) -> std::io::Result<Option<std::time::SystemTime>> {
        if_native_target! {{
            must_write_here_yet;
        }}
        if_browser_target! {{
            unsupported_browser_sync_operation!();
        }}
    }

    /// The creation date of a file or directory.
    /// This method returns asynchronously.
    /// 
    /// # Browser support
    /// 
    /// This method is not supported in the browser, thus returning always
    /// `Ok(None)`.
    /// 
    pub async fn creation_date_async(&self) -> std::io::Result<Option<std::time::SystemTime>> {
        if_native_target! {{
            must_write_here_yet;
        }}
        if_browser_target! {{
            Ok(None)
        }}
    }

    /// The modification date of a file or directory. This method
    /// returns synchronously.
    pub fn modification_date(&self) -> std::io::Result<Option<std::time::SystemTime>> {
        if_native_target! {{
            must_write_here_yet;
        }}
        if_browser_target! {{
            unsupported_browser_sync_operation!();
        }}
    }

    /// The modification date of a file or directory.
    /// This method returns asynchronously.
    /// 
    /// # Browser support
    /// 
    /// In the browser, this method returns `Ok(None)` for directories.
    /// 
    pub async fn modification_date_async(&self) -> std::io::Result<Option<std::time::SystemTime>> {
        if_native_target! {{
            must_write_here_yet;
        }}
        if_browser_target! {{
            target::browser::modification_date_async(self.path_omega()).await
        }}
    }

    /// The size of a file. This method returns synchronously.
    pub fn size(&self) -> std::io::Result<usize> {
        if_native_target! {{
            must_write_here_yet;
        }}
        if_browser_target! {{
            unsupported_browser_sync_operation!();
        }}
    }

    /// The size of a file. This method returns asynchronously.
    pub async fn size_async(&self) -> std::io::Result<usize> {
        if_native_target! {{
            must_write_here_yet;
        }}
        if_browser_target! {{
            target::browser::size_async(self.path_omega()).await
        }}
    }
}

#[allow(unused)]
macro unsupported_browser_operation {
    () => {
        panic!("Operation not supported in the browser");
    },
}

#[allow(unused)]
macro unsupported_browser_sync_operation {
    () => {
        panic!("Browser does not support synchronous file operations");
    },
}

#[allow(unused)]
macro unsupported_browser_filescheme_operation {
    () => {
        panic!("Browser does not support the 'file:' scheme");
    },
}

#[derive(Copy, Clone, PartialEq)]
enum FileScheme {
    File,
    App,
    AppStorage,
}

fn uri_to_native_path(uri: &str) -> String {
    assert!(uri.starts_with("file:"));
    cfg_if! {
        if #[cfg(target_os = "windows")] {
            return regex_replace!(r"^/{2,3}", &decode_uri(&uri[5..]), |_| "".to_owned()).into_owned();
        } else {
            return regex_replace!(r"^/{0,2}", &decode_uri(&uri[5..]), |_| "/".to_owned()).into_owned();
        }
    }
}

fn native_path_to_uri(path: &str) -> String {
    #[cfg(target_os = "windows")] {
        format!("file:///{}", encode_uri(&path))
    }
    #[cfg(not(target_os = "windows"))] {
        format!("file:/{}", encode_uri(&path))
    }
}

#[doc(hidden)]
#[allow(non_snake_case)]
pub async fn __agera_File_bootstrap() {
    if cfg!(debug_assertions) {
        // Pass
    } else {
        let _ = File::application_directory().create_directory_all_async().await;
        let _ = File::application_storage_directory().create_directory_all_async().await;
    }
}

fn application_directory() -> String {
    if_native_target! {{
        cfg_if! {
            if #[cfg(target_os = "android")] {
                let path = if let Some(p) = crate::target::application().external_data_path() { p } else { crate::target::application().internal_data_path().unwrap() };
                path.join("install").to_string_lossy().into_owned()
            } else {
                if cfg!(debug_assertions) {
                    std::env::current_dir().unwrap().to_str().unwrap().into()
                } else if #[cfg(target_os = "windows")] {
                    dirs::data_local_dir().unwrap().join(&crate::application::id()).to_string_lossy().into_owned()
                } else {
                    dirs::data_dir().unwrap().join(&crate::application::id()).join("install").to_string_lossy().into_owned()
                }
            }
        }
    }}
    if_browser_target! {{
        "/install".into()
    }}
}

fn application_storage_directory() -> String {
    if_native_target! {{
        cfg_if! {
            if #[cfg(target_os = "android")] {
                let path = if let Some(p) = crate::target::application().external_data_path() { p } else { crate::target::application().internal_data_path().unwrap() };
                path.join("storage").to_string_lossy().into_owned()
            } else if #[cfg(debug_assertions)] {
                std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("agera_sdk_build/storage").to_string_lossy().into_owned()
            } else if #[cfg(target_os = "windows")] {
                dirs::data_dir().unwrap().join(&crate::application::id()).to_string_lossy().into_owned()
            } else {
                dirs::data_dir().unwrap().join(&crate::application::id()).join("storage").to_string_lossy().into_owned()
            }
        }
    }}
    if_browser_target! {{
        "/storage".into()
    }}
}

fn downloads_directory() -> Option<String> {
    if_native_target! {{
        dirs::download_dir().map(|d| d.to_string_lossy().into_owned())
    }}
    if_browser_target! {{ None }}
}

fn documents_directory() -> Option<String> {
    if_native_target! {{
        dirs::document_dir().map(|d| d.to_string_lossy().into_owned())
    }}
    if_browser_target! {{ None }}
}

fn pictures_directory() -> Option<String> {
    if_native_target! {{
        dirs::picture_dir().map(|d| d.to_string_lossy().into_owned())
    }}
    if_browser_target! {{ None }}
}

fn music_directory() -> Option<String> {
    if_native_target! {{
        dirs::audio_dir().map(|d| d.to_string_lossy().into_owned())
    }}
    if_browser_target! {{ None }}
}

fn videos_directory() -> Option<String> {
    if_native_target! {{
        dirs::video_dir().map(|d| d.to_string_lossy().into_owned())
    }}
    if_browser_target! {{ None }}
}