use serde::{Deserialize, Serialize};
use lazy_regex::*;

pub mod errors;

#[derive(Deserialize, Serialize)]
pub struct ApplicationDescriptor {
    /// The application ID.
    pub id: String,
}

impl ApplicationDescriptor {
    /// Validates the application ID.
    pub fn validate_id(&self) -> Result<(), errors::InvalidId> {
        if regex_is_match!(r"[a-z][a-z0-9_]*(\.[a-z][a-z0-9_]*)+", &self.id) {
            Ok(())
        } else {
            Err(errors::InvalidId)
        }
    }
}