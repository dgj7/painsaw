use std::error::Error;
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub struct WindowingError(pub String);

impl Display for WindowingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.0)
    }
}

impl Error for WindowingError {}
