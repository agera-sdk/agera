use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct InvalidId;

impl Display for InvalidId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r"the application ID must match the Perl regular expression '[a-z][a-z0-9_]*(\.[a-z][a-z0-9_]*)+'")
    }
}

impl Error for InvalidId {}