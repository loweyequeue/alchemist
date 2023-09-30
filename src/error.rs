use std::fmt::Display;

use simply_colorful::Colorize;

use oh_no::{from_err, ErrorContext};

#[derive(Debug)]
pub struct AssertionError(pub String);

impl Display for AssertionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

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
    TomlParseError(ErrorContext<toml::de::Error>),
}

impl std::fmt::Display for AlchemistError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variant = match self {
            Self::IOError(_) => "IOError",
            Self::AssertionError(_) => "AssertionError",
            Self::TomlParseError(_) => "TomlParseError",
        }
        .to_string();
        write!(
            f,
            "{}{}{}{}{}",
            crate::cli::terminal::error_prefix(),
            "[".dimmed(),
            variant.dimmed().italic(),
            "]: ".dimmed(),
            self
        )
    }
}

from_err!(std::io::Error, AlchemistError, AlchemistError::IOError);
from_err!(
    AssertionError,
    AlchemistError,
    AlchemistError::AssertionError
);
from_err!(
    toml::de::Error,
    AlchemistError,
    AlchemistError::TomlParseError
);

pub type Result<T> = std::result::Result<T, AlchemistError>;
