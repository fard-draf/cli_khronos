use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Unvalid path")]
    UnvalidPath,

    #[error("Unvalid file")]
    UnvalidFile(#[from] io::Error),

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
}
