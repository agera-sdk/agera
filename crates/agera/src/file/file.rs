use crate::common::*;
use file_paths::{
    Path as FlexPath,
    PlatformPathVariant as FlexPathVariant,
};

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
                path: uri_to_native_path(&FlexPath::new_native(path_or_uri).to_string()),
            }
        } else if path_or_uri.starts_with("app:") {
            File {
                scheme: FileScheme::App,
                path: uri_to_app_path(&FlexPath::new_native(path_or_uri).to_string()),
            }
        } else if path_or_uri.starts_with("app-storage:") {
            File {
                scheme: FileScheme::AppStorage,
                path: uri_to_app_storage_path(&FlexPath::new_native(path_or_uri).to_string()),
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
                app_path_to_uri(&self.path)
            },
            FileScheme::AppStorage => {
                app_storage_path_to_uri(&self.path)
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
            FileScheme::File => FlexPathVariant::NATIVE,
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
        return decode_uri(uri);
    }
}

fn uri_to_app_path(uri: &str) -> String {
    assert!(uri.starts_with("app:"));
    return regex_replace!(r"^/{2}", &decode_uri(&uri[4..]), |_| "/".to_owned()).into_owned();
}

fn uri_to_app_storage_path(uri: &str) -> String {
    assert!(uri.starts_with("app-storage:"));
    return regex_replace!(r"^/{2}", &decode_uri(&uri[12..]), |_| "/".to_owned()).into_owned();
}

fn native_path_to_uri(path: &str) -> String {
    #[cfg(target_os = "windows")] {
        format!("file:///{}", encode_uri(&path))
    }
    #[cfg(not(target_os = "windows"))] {
        format!("file:{}", encode_uri(&path))
    }
}

fn app_path_to_uri(path: &str) -> String {
    format!("app:{}", encode_uri(&path))
}

fn app_storage_path_to_uri(path: &str) -> String {
    format!("app-storage:{}", encode_uri(&path))
}