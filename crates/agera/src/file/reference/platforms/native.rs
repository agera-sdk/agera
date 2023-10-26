use file_paths::FlexPath;

use crate::common::*;
use std::{io, path::PathBuf};

#[derive(Eq, PartialEq, Copy, Clone)]
enum EntryKind {
    File,
    Directory,
}

#[derive(Clone)]
pub struct FileSystemReference(pub PathBuf, EntryKind);

impl FileSystemReference {
    pub fn name(&self) -> String {
        FlexPath::new_native(&self.0.to_string_lossy().into_owned()).base_name()
    }

    pub fn as_directory(&self) -> Option<DirectoryReference> {
        if self.1 == EntryKind::Directory { Some(DirectoryReference(self.0.clone())) } else { None }
    }

    pub fn as_file(&self) -> Option<FileReference> {
        if self.1 == EntryKind::File { Some(FileReference(self.0.clone())) } else { None }
    }
}

#[derive(Clone)]
pub struct FileReference(pub PathBuf);

impl FileReference {
    pub async fn read_bytes(&self) -> io::Result<Bytes> {
        let bytes = tokio::fs::read(&self.0).await?;
        Ok(Bytes::from(bytes))
    }

    pub async fn read_utf8(&self) -> io::Result<String> {
        tokio::fs::read_to_string(&self.0).await
    }

    pub async fn write(&self, data: &[u8]) -> io::Result<()> {
        tokio::fs::write(&self.0, data).await
    }

    pub async fn modification_date(&self) -> io::Result<std::time::SystemTime> {
        tokio::fs::metadata(&self.0).await.and_then(|metadata| metadata.modified())
    }

    pub fn name(&self) -> String {
        FlexPath::new_native(&self.0.to_string_lossy().into_owned()).base_name()
    }

    pub async fn size(&self) -> io::Result<usize> {
        tokio::fs::metadata(&self.0).await.map(|metadata| metadata.len() as usize)
    }
}

impl From<FileReference> for FileSystemReference {
    fn from(value: FileReference) -> Self {
        FileSystemReference(value.0.clone(), EntryKind::File)
    }
}

#[derive(Clone)]
pub struct DirectoryReference(pub PathBuf);

impl DirectoryReference {
    pub fn name(&self) -> String {
        FlexPath::new_native(&self.0.to_string_lossy().into_owned()).base_name()
    }

    pub async fn entries(&self) -> io::Result<Vec<FileSystemReference>> {
        let mut listing_1 = tokio::fs::read_dir(&self.0).await?;
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
            let reference = FileSystemReference(entry.path(), if entry.file_type().await?.is_dir() { EntryKind::Directory } else { EntryKind::File });
            listing_2.push(reference);
        }
        Ok(listing_2)
    }

    pub async fn get_directory(&self, name: &str) -> io::Result<DirectoryReference> {
        if !is_valid_name(name) {
            return Err(io::Error::new(io::ErrorKind::InvalidFilename, "Invalid filename"));
        }
        let path = self.0.join(name);
        let metadata = tokio::fs::metadata(&path).await?;
        if metadata.is_dir() {
            Ok(DirectoryReference(path))
        } else {
            Err(io::Error::new(io::ErrorKind::NotADirectory, "Not a directory"))
        }
    }

    pub async fn get_directory_or_create(&self, name: &str) -> io::Result<DirectoryReference> {
        if !is_valid_name(name) {
            return Err(io::Error::new(io::ErrorKind::InvalidFilename, "Invalid filename"));
        }
        let path = self.0.join(name);

        let metadata = tokio::fs::metadata(&path).await;
        if let Err(error) = metadata {
            if error.kind() == io::ErrorKind::NotFound {
                tokio::fs::create_dir(&path).await?;
                return Ok(DirectoryReference(path));
            }
            return Err(error);
        }
        let metadata = metadata.unwrap();

        if metadata.is_dir() {
            Ok(DirectoryReference(path))
        } else {
            Err(io::Error::new(io::ErrorKind::NotADirectory, "Not a directory"))
        }
    }

    pub async fn get_file(&self, name: &str) -> io::Result<FileReference> {
        if !is_valid_name(name) {
            return Err(io::Error::new(io::ErrorKind::InvalidFilename, "Invalid filename"));
        }
        let path = self.0.join(name);
        let metadata = tokio::fs::metadata(&path).await?;
        if metadata.is_file() {
            Ok(FileReference(path))
        } else {
            Err(io::Error::new(io::ErrorKind::IsADirectory, "Entry is a directory"))
        }
    }

    pub async fn get_file_or_create(&self, name: &str) -> io::Result<FileReference> {
        if !is_valid_name(name) {
            return Err(io::Error::new(io::ErrorKind::InvalidFilename, "Invalid filename"));
        }
        let path = self.0.join(name);

        let metadata = tokio::fs::metadata(&path).await;
        if let Err(error) = metadata {
            if error.kind() == io::ErrorKind::NotFound {
                tokio::fs::write(&path, vec![]).await?;
                return Ok(FileReference(path));
            }
            return Err(error);
        }
        let metadata = metadata.unwrap();

        if metadata.is_file() {
            Ok(FileReference(path))
        } else {
            Err(io::Error::new(io::ErrorKind::IsADirectory, "Entry is a directory"))
        }
    }

    pub async fn delete_empty_directory(&self, name: &str) -> io::Result<()> {
        if !is_valid_name(name) {
            return Err(io::Error::new(io::ErrorKind::InvalidFilename, "Invalid filename"));
        }
        tokio::fs::remove_dir(self.0.join(name)).await
    }

    pub async fn delete_directory_all(&self, name: &str) -> io::Result<()> {
        if !is_valid_name(name) {
            return Err(io::Error::new(io::ErrorKind::InvalidFilename, "Invalid filename"));
        }
        tokio::fs::remove_dir_all(self.0.join(name)).await
    }

    pub async fn delete_file(&self, name: &str) -> io::Result<()> {
        if !is_valid_name(name) {
            return Err(io::Error::new(io::ErrorKind::InvalidFilename, "Invalid filename"));
        }
        tokio::fs::remove_file(self.0.join(name)).await
    }
}

impl From<DirectoryReference> for FileSystemReference {
    fn from(value: DirectoryReference) -> Self {
        FileSystemReference(value.0.clone(), EntryKind::Directory)
    }
}

fn is_valid_name(name: &str) -> bool {
    let path = std::path::PathBuf::from(name);
    let name_2 = path.file_name();
    if let Some(name_2) = name_2 { name == name_2 } else { false }
}