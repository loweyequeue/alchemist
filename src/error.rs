use std::fmt::Display;

use owo_colors::OwoColorize;

#[derive(Debug)]
pub struct ErrorContext<E: std::error::Error>(pub E, pub Option<String>);
pub trait ResultContext<T, E: std::error::Error> {
    fn error_msg<C: ToString>(self, msg: C) -> std::result::Result<T, ErrorContext<E>>;
}

impl<T, E: std::error::Error> ResultContext<T, E> for std::result::Result<T, E> {
    fn error_msg<C: ToString>(self, msg: C) -> std::result::Result<T, ErrorContext<E>> {
        self.map_err(|e| ErrorContext(e, Some(msg.to_string())))
    }
}

impl<E: std::error::Error> std::fmt::Display for ErrorContext<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(context) = &self.1 {
            write!(f, "{} ({})", context, self.0)
        } else {
            write!(f, "{}", self.0)
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct AssertionError(pub String);

impl std::error::Error for AssertionError {}
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

#[derive(Debug)]
pub enum AlchemistError {
    IOErrorVariant(ErrorContext<std::io::Error>),
    AssertionErrorVariant(ErrorContext<AssertionError>),
    TomlParseErrorVariant(ErrorContext<toml::de::Error>),
}

impl std::fmt::Display for AlchemistError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (variant, e) = match self {
            Self::IOErrorVariant(e) => ("IOError", e.to_string()),
            Self::AssertionErrorVariant(e) => ("AssertionError", e.to_string()),
            Self::TomlParseErrorVariant(e) => ("TomlParseError", e.to_string()),
        };
        write!(
            f,
            "{}{}{}{}{}",
            crate::cli::terminal::error_prefix(),
            "[".dimmed(),
            variant.dimmed().italic(),
            "]: ".dimmed(),
            e
        )
    }
}

impl PartialEq for AlchemistError {
    fn eq(&self, other: &Self) -> bool {
        return self.to_string() == other.to_string();
    }
}

impl From<AssertionError> for AlchemistError {
    fn from(value: AssertionError) -> Self {
        Self::AssertionErrorVariant(ErrorContext(value, None))
    }
}

impl From<std::io::Error> for AlchemistError {
    fn from(value: std::io::Error) -> Self {
        Self::IOErrorVariant(ErrorContext(value, None))
    }
}

impl From<toml::de::Error> for AlchemistError {
    fn from(value: toml::de::Error) -> Self {
        Self::TomlParseErrorVariant(ErrorContext(value, None))
    }
}

impl From<ErrorContext<AssertionError>> for AlchemistError {
    fn from(value: ErrorContext<AssertionError>) -> Self {
        Self::AssertionErrorVariant(value)
    }
}

impl From<ErrorContext<std::io::Error>> for AlchemistError {
    fn from(value: ErrorContext<std::io::Error>) -> Self {
        Self::IOErrorVariant(value)
    }
}

impl From<ErrorContext<toml::de::Error>> for AlchemistError {
    fn from(value: ErrorContext<toml::de::Error>) -> Self {
        Self::TomlParseErrorVariant(value)
    }
}

pub type Result<T> = std::result::Result<T, AlchemistError>;
