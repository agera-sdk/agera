use crate::{common::*, target::{if_native_target, unsupported_platform, if_browser_target}};
use file_paths::*;

/// Represents a file, either in the file system, application or
/// application storage directory.
/// 
/// The following URIs are supported:
/// 
/// * `file:` — A file located in the regular file system.
/// * `app:` — A file located in the application installation directory.
/// * `app-storage:` — A file located in the application storage directory.
#[derive(Clone, PartialEq)]
pub struct File {
    scheme: FileScheme,
    path: String,
}

impl File {
    /// Creates a file with a specified native path or URI.
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
            assert!(
                regex_is_match!(r"^[^:]+:", path_or_uri),
                "File::new() was supplied an unsupported URI scheme"
            );
            File {
                scheme: FileScheme::File,
                path: FlexPath::new_native(path_or_uri).to_string(),
            }
        }
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
            path: FlexPath::new(&self.path, self.flex_path_variant()).resolve(path).to_string(),
        }
    }

    fn flex_path_variant(&self) -> FlexPathVariant {
        match self.scheme {
            FileScheme::File => FlexPathVariant::native(),
            _ => FlexPathVariant::Common,
        }
    }
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

#[path = "target.rs"]
mod target;

#[doc(hidden)]
#[allow(non_snake_case)]
pub fn __agera_File_bootstrap() {
    if cfg!(debug_assertions) {
        // Pass
    } else {
        if_native_target! {{
            std::fs::create_dir_all(application_installation_directory()).unwrap();
            std::fs::create_dir_all(application_storage_directory()).unwrap();
        }}
    }
}

pub(crate) fn application_installation_directory() -> String {
    if_native_target! {{
        if cfg!(target_os = "android") {
            let path = if let Some(p) = crate::target::application().external_data_path() { p.to_string_lossy().into_owned() } else { crate::target::application().internal_data_path().unwrap().to_string_lossy().into_owned() };
            return FlexPath::new_common(&path).resolve(".install").to_string();
        } else {
            if cfg!(debug_assertions) {
                return std::env::current_dir().unwrap().to_str().unwrap().into();
            } else if cfg!(target_os = "windows") {
                return dirs::data_local_dir().unwrap().join(&crate::application::id()).to_string_lossy().into_owned();
            } else {
                unsupported_platform!();
            }
        }
    }}
    if_browser_target! {{
        panic!("Function must not be used in the web");
    }}
}

pub(crate) fn application_storage_directory() -> String {
    if_native_target! {{
        if cfg!(target_os = "android") {
            let path = if let Some(p) = crate::target::application().external_data_path() { p.to_string_lossy().into_owned() } else { crate::target::application().internal_data_path().unwrap().to_string_lossy().into_owned() };
            return FlexPath::new_common(&path).resolve(".storage").to_string();
        } else {
            if cfg!(debug_assertions) {
                return std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("agera_sdk_build/storage").to_string_lossy().into_owned();
            } else {
                unsupported_platform!();
            }
        }
    }}
    if_browser_target! {{
        panic!("Function must not be used in the web");
    }}
}

fn downloads_directory() -> Option<String> {
    todo!();
}

fn documents_directory() -> Option<String> {
    todo!();
}

fn pictures_directory() -> Option<String> {
    todo!();
}

fn music_directory() -> Option<String> {
    todo!();
}

fn videos_directory() -> Option<String> {
    todo!();
}