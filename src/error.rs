use std::fmt::Display;

use oh_no::{from_err, ErrorContext, ResultContext};

#[derive(Debug)]
pub struct AssertionError(pub String);

impl Display for AssertionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// impl<T> From<AssertionError> for Result<T> {
//     fn from(value: AssertionError) -> Self {
//         Err(AlchemistError::AssertionError(ErrorContext(value, None)))
//     }
// }

impl<T> From<AssertionError> for Result<T> {
    fn from(value: AssertionError) -> Self {
        Err(value.into())
    }
}

impl std::error::Error for AssertionError {}

#[derive(Debug)]
pub enum AlchemistError {
    IOError(ErrorContext<std::io::Error>),
    AssertionError(ErrorContext<AssertionError>),
}

from_err!(std::io::Error, AlchemistError, AlchemistError::IOError);
from_err!(
    AssertionError,
    AlchemistError,
    AlchemistError::AssertionError
);

pub type Result<T> = std::result::Result<T, AlchemistError>;
