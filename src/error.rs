use colored::Colorize;

#[derive(Debug, PartialEq, Eq)]
pub enum AlchemistErrorType {
    NoConfigFileError,
    ConfigParseError,
}

impl ToString for AlchemistErrorType {
    fn to_string(&self) -> String {
        match self {
            Self::NoConfigFileError => "NoConfigFileError",
            Self::ConfigParseError => "ConfigParseError",
        }
        .to_string()
    }
}

#[derive(Debug)]
pub struct AlchemistError {
    error_type: AlchemistErrorType,
    error_message: String,
}

impl std::fmt::Display for AlchemistError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{} {}",
            "[".dimmed(),
            "✘".red().bold(),
            "][".dimmed(),
            self.error_type.to_string().dimmed(),
            "]:".dimmed(),
            self.error_message
        )
    }
}

impl AlchemistError {
    pub fn new<T: ToString>(error_type: AlchemistErrorType, error_message: T) -> AlchemistError {
        AlchemistError {
            error_type,
            error_message: error_message.to_string(),
        }
    }
}

pub type Result<T> = std::result::Result<T, AlchemistError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exception_blatt() {
        let er = AlchemistError::new(AlchemistErrorType::ConfigParseError, "failed to parse");
        assert_eq!(er.error_type, AlchemistErrorType::ConfigParseError);
        let expected = format!(
            "[{}][ConfigParseError]: failed to parse",
            "✘".red().bold()
        );
        let test_str = format!("{}", er);
        assert_eq!(expected, test_str);
    }
}
