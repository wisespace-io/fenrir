use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct FenrirError {
    pub code: i16,
    pub message: String,
}

impl fmt::Display for FenrirError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error {}. {}", self.code, self.message)
    }
}

impl Error for FenrirError {}