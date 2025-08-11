use std::{env, path::PathBuf, process::Command};

use crate::errors::AppError;

pub fn edit_note(slug: &str) -> Result<(), AppError> {
    let mut path = PathBuf::from("notes");
    path.push(format!("{}.md", slug));

    if !path.exists() {
        return Err(AppError::NoteNotFound(path.display().to_string()));
    }

    let editor = env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());
    let status = Command::new(editor).arg(&path).status()?;

    if !status.success() {
        return Err(AppError::EditorExitedWithError);
    }

    Ok(())
}
