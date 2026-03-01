use thiserror::Error;

use crate::core::errors::CoreError;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Core(#[from] CoreError),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Editor exited with an error")]
    EditorExitedWithError,
}
