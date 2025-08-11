use std::{fs, path::PathBuf};

use termimad::MadSkin;

use crate::errors::AppError;

pub fn view_note(slug: &str) -> Result<(), AppError> {
    let mut path = PathBuf::from("notes");
    path.push(format!("{}.md", slug));

    if !path.exists() {
        return Err(AppError::NoteNotFound(path.display().to_string()));
    }

    let content = fs::read_to_string(&path)?;
    let skin = MadSkin::default();
    println!("📖 Viewing: {}\n", path.display());
    skin.print_text(&content);

    Ok(())
}
