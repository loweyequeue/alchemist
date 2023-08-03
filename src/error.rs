#[derive(Debug)]
pub enum AlchemistError {
    NoConfigFileError(String),
    ConfigParseError(String),
    CommandFailedError(String),
    InvalidSerialTask(String),
    CurrentDirIsInvalid(String),
    CLIError(String),
}

impl AlchemistError {
    pub fn kind(&self) -> String {
        match self {
            Self::NoConfigFileError(_) => "NoConfigFileError",
            Self::ConfigParseError(_) => "ConfigParseError",
            Self::CommandFailedError(_) => "CommandFailedError",
            Self::InvalidSerialTask(_) => "InvalidSerialTask",
            Self::CurrentDirIsInvalid(_) => "CurrentDirIsInvalid",
            Self::CLIError(_) => "CLIError",
        }
        .into()
    }

    pub fn inner(&self) -> String {
        // should be able to do this easier
        match self {
            Self::NoConfigFileError(e) => e,
            Self::ConfigParseError(e) => e,
            Self::CommandFailedError(e) => e,
            Self::InvalidSerialTask(e) => e,
            Self::CurrentDirIsInvalid(e) => e,
            Self::CLIError(e) => e,
        }
        .clone()
    }
}

impl Into<String> for AlchemistError {
    fn into(self) -> String {
        format!("{}({})", self.kind(), self.inner())
    }
}

pub type Result<T> = std::result::Result<T, AlchemistError>;
