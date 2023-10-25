use std::{fs, path::PathBuf};
use serde::{Deserialize, Serialize};
use lazy_regex::*;
use glob::glob;

pub mod errors;
use self::errors::ParsingError;

pub(crate) mod util;
use util::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct ApplicationDescriptor {
    /// The application ID.
    pub id: String,
    #[serde(rename = "installFiles")]
    pub install_files: ApplicationDescriptorInstallFiles,
}

/// The top-level `installFiles` property of the application descriptor.
#[derive(Deserialize, Serialize, Debug)]
pub struct ApplicationDescriptorInstallFiles {
    pub include: Vec<String>,
    pub exclude: Vec<String>,
}

impl ApplicationDescriptor {
    /// Parses application descriptor from a project's root directory.
    pub fn from_project(directory: &str) -> Result<Self, ParsingError> {
        let descriptor_path = PathBuf::from(directory).join("agera-application.json");
        let descriptor = fs::read_to_string(descriptor_path);
        if descriptor.is_err() {
            return Err(ParsingError::IoError(descriptor.unwrap_err()));
        }
        let descriptor = serde_json::from_str::<Self>(&descriptor.unwrap());
        if descriptor.is_err() {
            return Err(ParsingError::JsonError(descriptor.unwrap_err()));
        }
        let descriptor = descriptor.unwrap();

        let invalid_id: Option<errors::InvalidId> = descriptor.validate_id();
        if invalid_id.is_some() {
            return Err(ParsingError::ValidationErrors {
                invalid_id,
            });
        }

        Ok(descriptor)
    }

    /// Matches installation files from the current directory.
    pub fn glob_install_files(&self) -> Result<Vec<String>, (Vec<glob::PatternError>, Vec<glob::GlobError>)> {
        let mut result = vec![];
        let mut pattern_errors = vec![];
        let mut glob_errors = vec![];

        // installFiles.include
        for include_pattern in self.install_files.include.iter() {
            let paths = glob(&include_pattern);
            if paths.is_err() {
                pattern_errors.push(paths.unwrap_err());
                continue;
            }
            for path in paths.unwrap() {
                if path.is_err() {
                    glob_errors.push(path.unwrap_err());
                    continue;
                }
                result.push(path.unwrap().to_string_lossy().into_owned());
            }
        }

        // installFiles.exclude
        for exclude_pattern in self.install_files.exclude.iter() {
            let paths = glob(&exclude_pattern);
            if paths.is_err() {
                pattern_errors.push(paths.unwrap_err());
                continue;
            }
            for path in paths.unwrap() {
                if path.is_err() {
                    glob_errors.push(path.unwrap_err());
                    continue;
                }
                let path = path.unwrap().to_string_lossy().into_owned();
                loop {
                    if !result.remove_equals(&path) {
                        break;
                    }
                }
            }
        }

        if pattern_errors.is_empty() && glob_errors.is_empty() {
            Ok(result)
        } else {
            Err((pattern_errors, glob_errors))
        }
    }

    fn validate_id(&self) -> Option<errors::InvalidId> {
        if regex_is_match!(r"[a-z][a-z0-9_]*(\.[a-z][a-z0-9_]*)+", &self.id) { None } else { Some(errors::InvalidId) }
    }
}