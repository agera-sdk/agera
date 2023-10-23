use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum ParsingError {
    IoError(std::io::Error),
    JsonError(serde_json::Error),
    ValidationErrors {
        invalid_id: Option<InvalidId>,
    },
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsingError::IoError(error) => error.fmt(f),
            ParsingError::JsonError(error) => error.fmt(f),
            ParsingError::ValidationErrors { invalid_id } => {
                let mut r = vec![];
                if let Some(error) = invalid_id {
                    r.push(format!("* {}", error.to_string()));
                }
                write!(f, "{}", r.join("\n"))
            },
        }
    }
}

#[derive(Debug)]
pub struct InvalidId;

impl Display for InvalidId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r"the application ID must match the Perl regular expression '[a-z][a-z0-9_]*(\.[a-z][a-z0-9_]*)+'")
    }
}

impl Error for InvalidId {}