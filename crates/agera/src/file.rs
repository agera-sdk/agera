/*!
File API.
*/

use crate::{common::*, target::{if_native_target, if_browser_target}};
use file_paths::*;

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
    /// `app:` or `app-storage:`.
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

    /// Returns the native path of the file.
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

    /// Indicates whether a file or directory exists, synchronously.
    pub fn exists(&self) -> bool {
        if_native_target! {{
            must_write_here_yet;
        }}
        if_browser_target! {{
            unsupported_browser_sync_operation!();
        }}
    }

    /// Indicates whether a file or directory exists, asynchronously.
    pub async fn exists_async(&self) -> bool {
        if_native_target! {{
            must_write_here_yet;
        }}
        if_browser_target! {{
            match self.scheme {
                FileScheme::App => target::browser::exists_async(target::browser::within_application_directory(&self.path)).await,
                FileScheme::AppStorage => target::browser::exists_async(target::browser::within_application_storage_directory(&self.path)).await,
                FileScheme::File => false,
            }
        }}
    }

    /// Indicates whether the `File` object is a directory, synchronously.
    pub fn is_directory(&self) -> bool {
        if_native_target! {{
            must_write_here_yet;
        }}
        if_browser_target! {{
            unsupported_browser_sync_operation!();
        }}
    }

    /// Indicates whether the `File` object is a directory, asynchronously.
    pub async fn is_directory_async(&self) -> bool {
        if_native_target! {{
            must_write_here_yet;
        }}
        if_browser_target! {{
            match self.scheme {
                FileScheme::App => target::browser::is_directory_async(target::browser::within_application_directory(&self.path)).await,
                FileScheme::AppStorage => target::browser::is_directory_async(target::browser::within_application_storage_directory(&self.path)).await,
                FileScheme::File => false,
            }
        }}
    }

    /// Indicates whether the `File` object is a file, synchronously.
    pub fn is_file(&self) -> bool {
        if_native_target! {{
            must_write_here_yet;
        }}
        if_browser_target! {{
            unsupported_browser_sync_operation!();
        }}
    }

    /// Indicates whether the `File` object is a file, asynchronously.
    pub async fn is_file_async(&self) -> bool {
        if_native_target! {{
            must_write_here_yet;
        }}
        if_browser_target! {{
            match self.scheme {
                FileScheme::App => target::browser::is_file_async(target::browser::within_application_directory(&self.path)).await,
                FileScheme::AppStorage => target::browser::is_file_async(target::browser::within_application_storage_directory(&self.path)).await,
                FileScheme::File => false,
            }
        }}
    }

    /// Indicates whether the `File` object is a symbolic link, synchronously.
    pub fn is_symbolic_link(&self) -> bool {
        if_native_target! {{
            must_write_here_yet;
        }}
        if_browser_target! {{
            unsupported_browser_sync_operation!();
        }}
    }

    /// Indicates whether the `File` object is a symbolic link, asynchronously.
    pub async fn is_symbolic_link_async(&self) -> bool {
        if_native_target! {{
            must_write_here_yet;
        }}
        if_browser_target! {{
            false
        }}
    }

    /// Canonicalizes the file path, synchronously.
    pub fn canonicalize(&self) -> File {
        if_native_target! {{
            must_write_here_yet;
        }}
        if_browser_target! {{
            unsupported_browser_sync_operation!();
        }}
    }

    /// Canonicalizes the file path, asynchronously.
    pub fn canonicalize_async(&self) -> File {
        if_native_target! {{
            must_write_here_yet;
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
            panic!("Operation not supported in the browser");
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
            let flex_path = self.flex_path();
            let base_name = flex_path.base_name();
            let mut parent_path = flex_path.resolve("..").to_string();
            match self.scheme {
                FileScheme::App => { parent_path = target::browser::within_application_directory(&parent_path); },
                FileScheme::AppStorage => { parent_path = target::browser::within_application_storage_directory(&parent_path); },
                FileScheme::File => { unsupported_browser_filescheme_operation!(); },
            }
            target::browser::create_directory_async(parent_path, base_name).await
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
            let mut path = self.path.clone();
            match self.scheme {
                FileScheme::App => { path = target::browser::within_application_directory(&path); },
                FileScheme::AppStorage => { path = target::browser::within_application_storage_directory(&path); },
                FileScheme::File => { unsupported_browser_filescheme_operation!(); },
            }
            target::browser::create_directory_all_async(path).await
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
            let mut path = self.path.clone();
            match self.scheme {
                FileScheme::App => { path = target::browser::within_application_directory(&path); },
                FileScheme::AppStorage => { path = target::browser::within_application_storage_directory(&path); },
                FileScheme::File => { unsupported_browser_filescheme_operation!(); },
            }
            target::browser::read_bytes_async(path).await
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
            let mut path = self.path.clone();
            match self.scheme {
                FileScheme::App => { path = target::browser::within_application_directory(&path); },
                FileScheme::AppStorage => { path = target::browser::within_application_storage_directory(&path); },
                FileScheme::File => { unsupported_browser_filescheme_operation!(); },
            }
            target::browser::read_utf8_async(path).await
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
            let mut path = self.path.clone();
            match self.scheme {
                FileScheme::App => { path = target::browser::within_application_directory(&path); },
                FileScheme::AppStorage => { path = target::browser::within_application_storage_directory(&path); },
                FileScheme::File => { unsupported_browser_filescheme_operation!(); },
            }
            let listing_1 = target::browser::directory_listing_async(path).await?;
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
            let flex_path = self.flex_path();
            let base_name = flex_path.base_name();
            let mut parent_path = flex_path.resolve("..").to_string();
            match self.scheme {
                FileScheme::App => { parent_path = target::browser::within_application_directory(&parent_path); },
                FileScheme::AppStorage => { parent_path = target::browser::within_application_storage_directory(&parent_path); },
                FileScheme::File => { unsupported_browser_filescheme_operation!(); },
            }
            target::browser::delete_empty_directory_async(parent_path, base_name).await
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
            let flex_path = self.flex_path();
            let base_name = flex_path.base_name();
            let mut parent_path = flex_path.resolve("..").to_string();
            match self.scheme {
                FileScheme::App => { parent_path = target::browser::within_application_directory(&parent_path); },
                FileScheme::AppStorage => { parent_path = target::browser::within_application_storage_directory(&parent_path); },
                FileScheme::File => { unsupported_browser_filescheme_operation!(); },
            }
            target::browser::delete_directory_all_async(parent_path, base_name).await
        }}
    }
}

macro unsupported_browser_sync_operation {
    () => {
        panic!("Browser does not support synchronous file operations");
    },
}

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
    #[cfg(target_os = "windows")] {
        return regex_replace!(r"^/{2,3}", &decode_uri(&uri[5..]), |_| "".to_owned()).into_owned();
    }
    #[cfg(not(target_os = "windows"))] {
        return regex_replace!(r"^/{0,2}", &decode_uri(&uri[5..]), |_| "/".to_owned()).into_owned();
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

pub(crate) fn application_directory() -> String {
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
        panic!("Function must not be used in the web");
    }}
}

pub(crate) fn application_storage_directory() -> String {
    if_native_target! {{
        cfg_if! {
            if #[cfg(target_os = "android")] {
                let path = if let Some(p) = crate::target::application().external_data_path() { p } else { crate::target::application().internal_data_path().unwrap() };
                path.join("storage").to_string_lossy().into_owned()
            } else {
                if cfg!(debug_assertions) {
                    std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("agera_sdk_build/storage").to_string_lossy().into_owned()
                } else if #[cfg(target_os = "windows")] {
                    dirs::data_dir().unwrap().join(&crate::application::id()).to_string_lossy().into_owned()
                } else {
                    dirs::data_dir().unwrap().join(&crate::application::id()).join("storage").to_string_lossy().into_owned()
                }
            }
        }
    }}
    if_browser_target! {{
        panic!("Function must not be used in the web");
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