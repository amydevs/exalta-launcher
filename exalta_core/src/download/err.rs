use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct UpdateError(pub String);

impl fmt::Display for UpdateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UpdateError: {}", self.0)
    }
}

impl Error for UpdateError {}
