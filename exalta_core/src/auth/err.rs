use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct AuthError(pub String);

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AuthError: {}", self.0)
    }
}

impl Error for AuthError {}
