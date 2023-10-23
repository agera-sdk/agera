use std::{fs, path::PathBuf};
use serde::{Deserialize, Serialize};
use lazy_regex::*;

pub mod errors;
use self::errors::ParsingError;

#[derive(Deserialize, Serialize, Debug)]
pub struct ApplicationDescriptor {
    /// The application ID.
    pub id: String,
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

    fn validate_id(&self) -> Option<errors::InvalidId> {
        if regex_is_match!(r"[a-z][a-z0-9_]*(\.[a-z][a-z0-9_]*)+", &self.id) { None } else { Some(errors::InvalidId) }
    }
}