use std::fmt::Display;

use simply_colorful::Colorize;

use oh_no::{from_err, ErrorContext};

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum AlchemistError {
    IOErrorVariant(ErrorContext<std::io::Error>),
    AssertionErrorVariant(ErrorContext<AssertionError>),
    TomlParseErrorVariant(ErrorContext<toml::de::Error>),
}

impl AlchemistError {
    fn fmt_context(&self) -> String {
        match self {
            Self::IOErrorVariant(v) => v.to_string(),
            Self::AssertionErrorVariant(v) => v.to_string(),
            Self::TomlParseErrorVariant(v) => v.to_string(),
        }
    }
}

impl std::fmt::Display for AlchemistError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_in_variant = match self {
            Self::IOErrorVariant(_) => "IOError",
            Self::AssertionErrorVariant(_) => "AssertionError",
            Self::TomlParseErrorVariant(_) => "TomlParseError",
        }
        .to_string();
        write!(
            f,
            "{}{}{}{}{}",
            crate::cli::terminal::error_prefix(),
            "[".dimmed(),
            error_in_variant.dimmed().italic(),
            "]: ".dimmed(),
            self.fmt_context()
        )
    }
}

from_err!(
    std::io::Error,
    AlchemistError,
    AlchemistError::IOErrorVariant
);
from_err!(
    AssertionError,
    AlchemistError,
    AlchemistError::AssertionErrorVariant
);
from_err!(
    toml::de::Error,
    AlchemistError,
    AlchemistError::TomlParseErrorVariant
);

pub type Result<T> = std::result::Result<T, AlchemistError>;
