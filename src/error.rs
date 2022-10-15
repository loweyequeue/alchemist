#[derive(Debug, PartialEq, Eq)]
pub enum AlchemistErrorType {
    NoConfigFileError,
    ConfigParseError,
    CommandFailedError,
    InvalidSerialTask,
    CurrentDirIsInvalid,
}

impl AlchemistErrorType {
    pub fn with_message<T: ToString>(self, message: T) -> AlchemistError {
        AlchemistError {
            error_type: self,
            error_message: message.to_string(),
        }
    }
}

impl ToString for AlchemistErrorType {
    fn to_string(&self) -> String {
        match self {
            Self::NoConfigFileError => "NoConfigFileError",
            Self::ConfigParseError => "ConfigParseError",
            Self::CommandFailedError => "CommandFailedError",
            Self::InvalidSerialTask => "InvalidSerialTask",
            Self::CurrentDirIsInvalid => "CurrentDirIsInvalid",
        }
        .to_string()
    }
}

#[derive(Debug)]
pub struct AlchemistError {
    pub error_type: AlchemistErrorType,
    pub error_message: String,
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
