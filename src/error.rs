use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Unvalid path")]
    UnvalidPath,
}
