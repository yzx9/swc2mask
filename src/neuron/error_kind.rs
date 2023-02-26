use std::fmt;

#[derive(Debug)]
pub struct RootNotFoundError;

impl std::error::Error for RootNotFoundError {}

impl fmt::Display for RootNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Root not found")
    }
}
