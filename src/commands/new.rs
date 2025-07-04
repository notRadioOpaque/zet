use crate::utils::frontmatter::Frontmatter;
use crate::utils::utils::{build_note_content, slugify, validate_tags};
use chrono::Local;
use std::error::Error;
use std::fs;
use std::path::Path;

pub fn create_note(title: &str, tags: Option<&str>) -> Result<(), Box<dyn Error>> {
    if title.trim().is_empty() {
        return Err("Title cannot be empty".into());
    }

    let date = Local::now().format("%y-%m-%d").to_string();
    let slug = slugify(title);
    let file_path = format!("notes/{}.md", slug);

    let tag_list = match tags {
        Some(t) => validate_tags(t)?,
        _ => vec![],
    };

    let frontmatter = Frontmatter {
        title: title.to_string(),
        date,
        tags: tag_list,
    };

    let body = format!("# {}\n\n<!-- Start writing your note below -->\n", title);
    let content = build_note_content(&frontmatter, &body)?;

    if !Path::new("notes").exists() {
        fs::create_dir("notes").expect("Failed to create directory");
    }

    fs::write(&file_path, content)?;
    println!("🎉 note created: {}", file_path);

    Ok(())
}
