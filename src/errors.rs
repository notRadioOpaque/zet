use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    // note creation errors
    #[error("title cannot be empty")]
    EmptyTitle,

    #[error("invalid tags: {0}")]
    InvalidTags(String),

    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("frontmatter build error: {0}")]
    FrontmatterBuild(String),

    #[error("Note not found: {0}")]
    NoteNotFound(String),

    #[error("Editor exited with an error")]
    EditorExitedWithError,

    #[error("Failed to serialize frontmatter for: {0}")]
    FrontmatterSerialize(serde_yaml::Error),
    // TODO: TUI errors
}
