use std::{env, error::Error, path::PathBuf, process::Command};

pub fn edit_note(slug: &str) -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from("notes");
    path.push(format!("{}.md", slug));

    if !path.exists() {
        return Err(format!("Note not found: {}", path.display()).into());
    }

    let editor = env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());
    let status = Command::new(editor).arg(&path).status()?;

    if !status.success() {
        return Err("Editor exited with an error".into());
    }

    Ok(())
}
