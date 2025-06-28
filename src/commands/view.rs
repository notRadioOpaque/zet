use std::{error::Error, fs, path::PathBuf};

use termimad::MadSkin;

pub fn view_note(slug: &str) -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from("notes");
    path.push(format!("{}.md", slug));

    if !path.exists() {
        return Err(format!("Note not found: {}", path.display()).into());
    }

    let content = fs::read_to_string(&path)?;
    let skin = MadSkin::default();
    println!("📖 Viewing: {}\n", path.display());
    skin.print_text(&content);

    Ok(())
}
