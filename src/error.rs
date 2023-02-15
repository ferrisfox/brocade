use std::{
    error::Error,
    fmt::{self, Display},
};

#[derive(Debug)]
pub struct BrocadeError;

impl Display for BrocadeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "brocade error")
    }
}

impl Error for BrocadeError {}

#[derive(Debug)]
pub struct GTINError(pub(crate) &'static str);

impl Display for GTINError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "gtin error: {}", self.0)
    }
}

impl std::error::Error for GTINError {}
