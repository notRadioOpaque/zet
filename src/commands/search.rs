// use super::view::view_note;
use crate::utils::frontmatter::parse_frontmatter;
// use skim::prelude::*;
use std::{error::Error, fs, path::Path};

pub fn interactive_search() -> Result<(), Box<dyn Error>> {
    let notes_dir = Path::new("notes");

    if !notes_dir.exists() {
        return Err("Notes directory does not exist".into());
    }

    let mut entries = Vec::new();

    for entry in fs::read_dir(notes_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) != Some("md") {
            continue;
        }

        let slug = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default();

        let content = fs::read_to_string(&path)?;

        let frontmatter = match parse_frontmatter(&content, slug) {
            Ok(fm) => fm,
            Err(_) => continue, // skips if frontmatter is invalid or absent
        };

        // format: slug => title preview
        entries.push(format!("{} => {}", slug, frontmatter.title));
    }

    if entries.is_empty() {
        println!("No notes found.");

        return Ok(());
    }

    const PREVIEW_CMD: &str = "command -v bat >/dev/null && bat --style=plain --color=always notes/{1}.md || cat notes/{1}.md";

    // let options = SkimOptionsBuilder::default()
    //     .height("60%".to_string())
    //     .prompt("🔍 Search notes: ".to_string())
    //     .preview(Some(PREVIEW_CMD.to_string()))
    //     .preview_window("right:50%".to_string())
    //     .multi(false)
    //     .build()?;

    // let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();
    //
    // for entry in entries {
    //     let _ = tx.send(Arc::new(entry));
    // }
    //
    // drop(tx);

    // let selected = Skim::run_with(&options, Some(rx))
    //     .map(|out| out.selected_items)
    //     .unwrap_or_default();
    //
    // if let Some(item) = selected.first() {
    //     // parse "slug ==> title"
    //     let rough_format = item.output();
    //     let slug = rough_format.split("=>").next().unwrap().trim();
    //     view_note(slug)?
    // };

    Ok(())
}
