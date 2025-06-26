use std::error::Error;
use std::fs;
use std::path::Path;

pub fn list_notes() -> Result<(), Box<dyn Error>> {
    let notes_dir = Path::new("notes");

    if !notes_dir.exists() {
        println!("📭 No notes found. Try creating one first with `zet new <title>`.");

        return Ok(());
    }

    let entries = fs::read_dir(notes_dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().extension().and_then(|s| s.to_str()) == Some("md"))
        .collect::<Vec<_>>();

    if entries.is_empty() {
        println!("📭 No notes found.");
    } else {
        println!("🗒️  Notes:\n");

        for (i, entry) in entries.iter().enumerate() {
            println!("{}. {}", i + 1, entry.file_name().to_string_lossy());
        }
    }

    Ok(())
}
