use std::error::Error;
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub struct WindowError(pub String);

impl Display for WindowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.0)
    }
}

impl Error for WindowError {}
