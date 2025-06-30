use std::{error::Error, fs, path::Path};

use skim::prelude::*;

use super::view::view_note;

pub fn interactive_search() -> Result<(), Box<dyn Error>> {
    let notes_dir = Path::new("notes");

    if !notes_dir.exists() {
        return Err("Notes directory does not exist".into());
    }

    let mut entries = Vec::new();

    for entry in fs::read_dir(notes_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("md") {
            let slug = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or_default();

            let file_content = fs::read_to_string(&path).unwrap_or_default();

            let mut title = "";
            let mut in_frontmatter = false;

            for line in file_content.lines() {
                let line = line.trim();

                if line == "---" {
                    if !in_frontmatter {
                        in_frontmatter = true;
                        continue;
                    } else {
                        break;
                    }
                }

                if in_frontmatter && line.starts_with("title:") {
                    title = line.trim_start_matches("title:").trim();
                    break;
                }
            }

            // format: slug === title preview
            entries.push(format!("{} => {}", slug, title));
        }
    }

    if entries.is_empty() {
        println!("No notes found.");

        return Ok(());
    }

    let options = SkimOptionsBuilder::default()
        .height("60%".to_string())
        .prompt("🔍 Search notes: ".to_string())
        .preview(None)
        .multi(false)
        .build()?;

    let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();

    for entry in entries {
        let _ = tx.send(Arc::new(entry));
    }

    drop(tx);

    let selected = Skim::run_with(&options, Some(rx))
        .map(|out| out.selected_items)
        .unwrap_or_default();

    if let Some(item) = selected.first() {
        // parse "slug ==> title"
        let rough_format = item.output();
        let slug = rough_format.split("=>").next().unwrap().trim();
        view_note(slug)?
    };

    Ok(())
}
