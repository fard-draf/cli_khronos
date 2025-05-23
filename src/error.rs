use thiserror::Error;

//=============================================================APP_ERROR
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Unvalid path")]
    UnvalidPath,

    #[error("Unvalid file")]
    UnvalidFile(#[from] std::io::Error),

    #[error("Empty file")]
    EmptyFile,

    #[error("Unvalid format")]
    UnvalidFormat,

    #[error("Unvalid TaskDateTime format")]
    UnvalidDateTimeFormat,

    #[error("Unvalid TaskTitle format")]
    UnvalidTaskTitleFormat,

    #[error("Unvalid TaskTag format")]
    UnvalidTaskTagFormat,

    #[error("Unvalid TaskName")]
    UnvalidTaskName,

    #[error("Regex Error")]
    RegexError(#[from] regex::Error),

    #[error("Chrono Error")]
    ChronoError(#[from] chrono::ParseError),

    #[error("Uuid Parsing Error")]
    UuidError(#[from] uuid::Error),

    #[error("Domain Error")]
    DomainError(#[from] crate::error::DomainError),

    #[error("Serde Error")]
    SerdeError(#[from] serde),
}

//=============================================================DOMAIN_ERROR

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Domain Error")]
    DomainError,

    #[error("Unvalid Path")]
    UnvalidPath,

    #[error("Unvalid TaskTitle Format")]
    UnvalidTaskTitleFormat,

    #[error("Unvalid TaskTag Format")]
    UnvalidTaskTagFormat,

    #[error("Unvalid Format")]
    UnvalidFormat,

    #[error("Unvalid Time Range")]
    UnvalidTimeRange,

    #[error("Unvalid Parsing Uuid")]
    UnvalidParsingUuid,
}
