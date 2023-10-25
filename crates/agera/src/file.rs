/*!
File API.
*/

use crate::{common::*, target::{if_native_target, if_browser_target}};
use file_paths::*;

#[allow(unused)]
use std::path::Path;

pub(crate) mod target;

/// Represents a file or directory path, either in the native file system, application or
/// application storage directory.
/// 
/// The following URIs are supported when constructing a `File` object:
/// 
/// * `file:` — A file located in the native file system.
/// * `app:` — A file located in the application installation directory.
/// * `app-storage:` — A file located in the application storage directory.
/// 
/// # Browser support
/// 
/// * Synchronous operations are supported on all platforms except for the browser.
/// Synchronous operations are expected to panic when running in the browser.
/// * The `file:` scheme is not supported in the browser. If it is required
/// for the application to pick user files or directories, consider using
/// file pickers and thus `FileReference` and `DirectoryReference`.
///
#[derive(Clone, Eq, PartialEq)]
pub struct File {
    scheme: FileScheme,
    path: String,
}

impl File {
    /// Creates a file with a specified native path or URI.
    /// `path_or_uri` is treated as an URI if it starts with either
    /// `file:`, `app:` or `app-storage:`.
    /// 
    /// If this constructor is given a non URI, it is taken as a
    /// `file:` native path. If that native path is not absolute,
    /// this native path is reassigned as the current working directory
    /// resolved to that native path.
    ///
    pub fn new(path_or_uri: &str) -> File {
        if path_or_uri.starts_with("file:") {
            File {
                scheme: FileScheme::File,
                path: File::current_directory().resolve_path(&uri_to_native_path(path_or_uri)).path,
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
                path: File::current_directory().resolve_path(path_or_uri).path,
            }
        }
    }

    /// The current working directory. The result of this function is non-constant.
    /// 
    /// # Browser support
    /// 
    /// This function is not supported in the browser and may thus panic.
    /// 
    pub fn current_directory() -> File {
        if_native_target! {{
            Self {
                scheme: FileScheme::File,
                path: std::env::current_dir().unwrap().to_string_lossy().into_owned(),
            }
        }}
        if_browser_target! {{
            unsupported_browser_operation!();
        }}
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

    /// Finds the relative path from this file or directory to `other`.
    ///
    /// # Panics
    /// 
    /// Panics if any of the `File` objects have a different scheme.
    /// 
    /// # Example
    /// 
    /// ```
    /// use agera::file::*;
    /// 
    /// let file_1 = File::new("file:///C:/Users/John/Documents/foo.svg");
    /// let file_2 = File::new("file:///C:/Users/John/Documents/bar.svg");
    /// assert_eq!("../bar.svg", file_1.relative(&file_2));
    /// ```
    ///
    pub fn relative(&self, other: &File) -> String {
        assert_eq!(self.scheme, other.scheme, "Files have different scheme");
        self.flex_path().relative(&other.path)
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
    /// 
    /// # Browser support
    ///
    /// This is a synchronous operation, therefore it is not supported
    /// in the browser.
    ///
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
    /// 
    /// # Browser support
    ///
    /// This is a synchronous operation, therefore it is not supported
    /// in the browser.
    ///
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
    /// 
    /// # Browser support
    ///
    /// This is a synchronous operation, therefore it is not supported
    /// in the browser.
    ///
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
    /// 
    /// # Browser support
    ///
    /// This is a synchronous operation, therefore it is not supported
    /// in the browser.
    /// 
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
    /// # Browser support
    ///
    /// This is a synchronous operation, therefore it is not supported
    /// in the browser.
    /// 
    pub fn canonicalize(&self) -> File {
        if_native_target! {{
            if self.scheme != FileScheme::File {
                return self.clone();
            }
            if let Some(result) = Path::new(&self.path_omega()).canonicalize().ok().map(|result| result.to_string_lossy().into_owned()) {
                return File { scheme: FileScheme::File, path: result };
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

    /// Copies a file to another path specified by `location`,
    /// overriding any contents at `location`. This is a synchronous operation.
    /// 
    /// # Browser support
    ///
    /// This is a synchronous operation, therefore it is not supported
    /// in the browser.
    ///
    pub fn copy_file_contents_to(&self, location: &File) -> std::io::Result<()> {
        if_native_target! {{
            std::fs::copy(&self.path_omega(), &location.path_omega())?;
            Ok(())
        }}
        if_browser_target! {{
            let _ = location;
            unsupported_browser_sync_operation!();
        }}
    }

    /// Copies a file to another path specified by `location`,
    /// overriding any contents at `location`. This is an asynchronous operation.
    /// 
    /// # Browser support
    /// 
    /// This operation is currently not supported in the browser
    /// and thus should panic.
    ///
    pub async fn copy_file_contents_to_async(&self, location: &File) -> std::io::Result<()> {
        if_native_target! {{
            tokio::fs::copy(&self.path_omega(), &location.path_omega()).await?;
            Ok(())
        }}
        if_browser_target! {{
            let _ = location;
            unsupported_browser_operation!();
        }}
    }

    /// Creates an empty directory synchronously.
    /// 
    /// # Browser support
    ///
    /// This is a synchronous operation, therefore it is not supported
    /// in the browser.
    ///
    pub fn create_directory(&self) -> std::io::Result<()> {
        if_native_target! {{
            std::fs::create_dir(&self.path_omega())
        }}
        if_browser_target! {{
            unsupported_browser_sync_operation!();
        }}
    }

    /// Creates an empty directory asynchronously.
    pub async fn create_directory_async(&self) -> std::io::Result<()> {
        if_native_target! {{
            tokio::fs::create_dir(&self.path_omega()).await
        }}
        if_browser_target! {{
            target::browser::create_directory_async(self.parent().path_omega(), self.flex_path().base_name()).await
        }}
    }

    /// Creates a directory and its parent directories synchronously.
    /// 
    /// # Browser support
    ///
    /// This is a synchronous operation, therefore it is not supported
    /// in the browser.
    /// 
    pub fn create_directory_all(&self) -> std::io::Result<()> {
        if_native_target! {{
            std::fs::create_dir_all(&self.path_omega())
        }}
        if_browser_target! {{
            unsupported_browser_sync_operation!();
        }}
    }

    /// Creates a directory and its parent directories asynchronously.
    pub async fn create_directory_all_async(&self) -> std::io::Result<()> {
        if_native_target! {{
            tokio::fs::create_dir_all(&self.path_omega()).await
        }}
        if_browser_target! {{
            target::browser::create_directory_all_async(self.path_omega()).await
        }}
    }

    /// Reads the bytes from a file synchronously.
    /// 
    /// # Browser support
    ///
    /// This is a synchronous operation, therefore it is not supported
    /// in the browser.
    ///
    pub fn read_bytes(&self) -> std::io::Result<Bytes> {
        if_native_target! {{
            std::fs::read(&self.path_omega()).map(|data| Bytes::from(data))
        }}
        if_browser_target! {{
            unsupported_browser_sync_operation!();
        }}
    }

    /// Reads the bytes from a file asynchronously.
    pub async fn read_bytes_async(&self) -> std::io::Result<Bytes> {
        if_native_target! {{
            tokio::fs::read(&self.path_omega()).await.map(|data| Bytes::from(data))
        }}
        if_browser_target! {{
            target::browser::read_bytes_async(self.path_omega()).await
        }}
    }

    /// Reads an UTF-8 string from a file synchronously.
    /// 
    /// # Browser support
    ///
    /// This is a synchronous operation, therefore it is not supported
    /// in the browser.
    /// 
    pub fn read_utf8(&self) -> std::io::Result<String> {
        if_native_target! {{
            std::fs::read_to_string(&self.path_omega())
        }}
        if_browser_target! {{
            unsupported_browser_sync_operation!();
        }}
    }

    /// Reads an UTF-8 string from a file asynchronously.
    pub async fn read_utf8_async(&self) -> std::io::Result<String> {
        if_native_target! {{
            tokio::fs::read_to_string(&self.path_omega()).await
        }}
        if_browser_target! {{
            target::browser::read_utf8_async(self.path_omega()).await
        }}
    }

    /// Returns entries from a directory, synchronously.
    /// 
    /// # Browser support
    ///
    /// This is a synchronous operation, therefore it is not supported
    /// in the browser.
    /// 
    pub fn directory_listing(&self) -> std::io::Result<Vec<File>> {
        if_native_target! {{
            let listing_1 = std::fs::read_dir(&self.path_omega())?;
            let mut listing_2 = vec![];
            for entry in listing_1 {
                if entry.is_err() {
                    continue;
                }
                let entry_name = entry.unwrap().file_name();
                listing_2.push(self.resolve_path(&entry_name.to_string_lossy().into_owned()));
            }
            Ok(listing_2)
        }}
        if_browser_target! {{
            unsupported_browser_sync_operation!();
        }}
    }

    /// Returns entries from a directory, asynchronously.
    pub async fn directory_listing_async(&self) -> std::io::Result<Vec<File>> {
        if_native_target! {{
            let mut listing_1 = tokio::fs::read_dir(&self.path_omega()).await?;
            let mut listing_2 = vec![];
            loop {
                let entry = listing_1.next_entry().await;
                if entry.is_err() {
                    continue;
                }
                let entry = entry.unwrap();
                if entry.is_none() {
                    break;
                }
                let entry = entry.unwrap();
                let entry_name = entry.file_name();
                listing_2.push(self.resolve_path(&entry_name.to_string_lossy().into_owned()));
            }
            Ok(listing_2)
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
    /// 
    /// # Browser support
    ///
    /// This is a synchronous operation, therefore it is not supported
    /// in the browser.
    /// 
    pub fn delete_empty_directory(&self) -> std::io::Result<()> {
        if_native_target! {{
            std::fs::remove_dir(&self.path_omega())
        }}
        if_browser_target! {{
            unsupported_browser_sync_operation!();
        }}
    }

    /// Deletes an empty directory asynchronously.
    pub async fn delete_empty_directory_async(&self) -> std::io::Result<()> {
        if_native_target! {{
            tokio::fs::remove_dir(&self.path_omega()).await
        }}
        if_browser_target! {{
            target::browser::delete_empty_directory_async(self.parent().path_omega(), self.flex_path().base_name()).await
        }}
    }

    /// Deletes a directory recursively synchronously.
    /// 
    /// # Browser support
    ///
    /// This is a synchronous operation, therefore it is not supported
    /// in the browser.
    /// 
    pub fn delete_directory_all(&self) -> std::io::Result<()> {
        if_native_target! {{
            std::fs::remove_dir_all(&self.path_omega())
        }}
        if_browser_target! {{
            unsupported_browser_sync_operation!();
        }}
    }

    /// Deletes a directory recursively asynchronously.
    pub async fn delete_directory_all_async(&self) -> std::io::Result<()> {
        if_native_target! {{
            tokio::fs::remove_dir_all(&self.path_omega()).await
        }}
        if_browser_target! {{
            target::browser::delete_directory_all_async(self.parent().path_omega(), self.flex_path().base_name()).await
        }}
    }

    /// Deletes a file synchronously.
    /// 
    /// # Browser support
    ///
    /// This is a synchronous operation, therefore it is not supported
    /// in the browser.
    /// 
    pub fn delete_file(&self) -> std::io::Result<()> {
        if_native_target! {{
            std::fs::remove_file(&self.path_omega())
        }}
        if_browser_target! {{
            unsupported_browser_sync_operation!();
        }}
    }

    /// Deletes a file asynchronously.
    pub async fn delete_file_async(&self) -> std::io::Result<()> {
        if_native_target! {{
            tokio::fs::remove_file(&self.path_omega()).await
        }}
        if_browser_target! {{
            target::browser::delete_file_async(self.parent().path_omega(), self.flex_path().base_name()).await
        }}
    }

    /// Moves a file or directory from its existing path to the path `path`, synchronously.
    /// This method overrides any file contents present at the path `path`.
    /// 
    /// # Browser support
    ///
    /// This is a synchronous operation, therefore it is not supported
    /// in the browser.
    /// 
    /// # Example
    /// 
    /// ```
    /// use agera::file::*;
    /// 
    /// // Rename a.txt to b.txt
    /// let a_txt = File::new("a.txt");
    /// let b_txt = File::new("b.txt");
    /// a_txt.move_to(&b_txt)?;
    /// ```
    /// 
    pub fn move_to(&self, path: &File) -> std::io::Result<()> {
        if_native_target! {{
            std::fs::rename(&self.path_omega(), &path.path_omega())
        }}
        if_browser_target! {{
            let _ = path;
            unsupported_browser_sync_operation!();
        }}
    }

    /// Moves a file or directory from its existing path to the path `path`, asynchronously.
    /// This method overrides any file contents present at the path `path`.
    /// 
    /// # Browser support
    /// 
    /// This operation is currently not supported in the browser
    /// and thus should panic.
    /// 
    /// # Example
    /// 
    /// ```
    /// use agera::file::*;
    /// 
    /// // Rename a.txt to b.txt
    /// let a_txt = File::new("a.txt");
    /// let b_txt = File::new("b.txt");
    /// a_txt.move_to_async(&b_txt).await?;
    /// ```
    ///
    pub async fn move_to_async(&self, path: &File) -> std::io::Result<()> {
        if_native_target! {{
            tokio::fs::rename(&self.path_omega(), &path.path_omega()).await
        }}
        if_browser_target! {{
            let _ = path;
            unsupported_browser_operation!();
        }}
    }

    /// Writes data to a file synchronously.
    /// 
    /// # Browser support
    ///
    /// This is a synchronous operation, therefore it is not supported
    /// in the browser.
    ///
    pub fn write<T: AsRef<[u8]>>(&self, data: T) -> std::io::Result<()> {
        if_native_target! {{
            std::fs::write(&self.path_omega(), data)
        }}
        if_browser_target! {{
            let _ = data;
            unsupported_browser_sync_operation!();
        }}
    }

    /// Writes data to a file asynchronously.
    pub async fn write_async<T: AsRef<[u8]>>(&self, data: T) -> std::io::Result<()> {
        if_native_target! {{
            tokio::fs::write(&self.path_omega(), data).await
        }}
        if_browser_target! {{
            target::browser::write_async(self.path_omega(), data.as_ref()).await
        }}
    }

    /// The creation date of a file or directory. This method returns synchronously.
    /// 
    /// # Browser support
    ///
    /// This is a synchronous operation, therefore it is not supported
    /// in the browser.
    ///
    pub fn creation_date(&self) -> std::io::Result<Option<std::time::SystemTime>> {
        if_native_target! {{
            std::fs::metadata(&self.path_omega()).map(|metadata| metadata.created().ok())
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
            tokio::fs::metadata(&self.path_omega()).await.map(|metadata| metadata.created().ok())
        }}
        if_browser_target! {{
            Ok(None)
        }}
    }

    /// The modification date of a file or directory. This method
    /// returns synchronously.
    /// 
    /// # Browser support
    ///
    /// This is a synchronous operation, therefore it is not supported
    /// in the browser.
    ///
    pub fn modification_date(&self) -> std::io::Result<Option<std::time::SystemTime>> {
        if_native_target! {{
            std::fs::metadata(&self.path_omega()).map(|metadata| metadata.modified().ok())
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
            tokio::fs::metadata(&self.path_omega()).await.map(|metadata| metadata.modified().ok())
        }}
        if_browser_target! {{
            target::browser::modification_date_async(self.path_omega()).await
        }}
    }

    /// The size of a file. This method returns synchronously.
    /// 
    /// # Browser support
    ///
    /// This is a synchronous operation, therefore it is not supported
    /// in the browser.
    ///
    pub fn size(&self) -> std::io::Result<usize> {
        if_native_target! {{
            std::fs::metadata(&self.path_omega()).map(|metadata| metadata.len() as usize)
        }}
        if_browser_target! {{
            unsupported_browser_sync_operation!();
        }}
    }

    /// The size of a file. This method returns asynchronously.
    pub async fn size_async(&self) -> std::io::Result<usize> {
        if_native_target! {{
            tokio::fs::metadata(&self.path_omega()).await.map(|metadata| metadata.len() as usize)
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

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
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
            } else if #[cfg(debug_assertions)] {
                std::env::current_dir().unwrap().to_str().unwrap().into()
            } else if #[cfg(target_os = "windows")] {
                dirs::data_local_dir().unwrap().join(&crate::application::id()).to_string_lossy().into_owned()
            } else {
                dirs::data_dir().unwrap().join(&crate::application::id()).join("install").to_string_lossy().into_owned()
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

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        #[path = "./file/file_reference_target/browser.rs"]
        mod file_reference_target;

        #[path = "./file/directory_reference_target/browser.rs"]
        mod directory_reference_target;
    } else {
        #[path = "./file/file_reference_target/native.rs"]
        mod file_reference_target;

        #[path = "./file/directory_reference_target/native.rs"]
        mod directory_reference_target;
    }
}

/// `FileReference` represents a reference to a file.
/// 
/// # Browser support
/// 
/// Unlike with `File` objects, all operations on `FileReference` are asynchronous and are
/// designed to be compatible with the browser.
///
#[derive(Clone)]
pub struct FileReference {
    inner: file_reference_target::FileReference,
}

/// `DirectoryReference` represents a reference to a directory.
/// 
/// # Browser support
/// 
/// Unlike with `File` objects, all operations on `DirectoryReference` are asynchronous and are
/// designed to be compatible with the browser.
///
#[derive(Clone)]
pub struct DirectoryReference {
    inner: directory_reference_target::DirectoryReference,
}