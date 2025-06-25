use crate::utils::slugify;
use chrono::Local;
use std::error::Error;
use std::fs;
use std::path::Path;

pub fn create_note(title: &str) -> Result<(), Box<dyn Error>> {
    if title.trim().is_empty() {
        return Err("Title cannot be empty".into());
    }

    let date = Local::now().format("%y-%m-%d").to_string();
    let slug = slugify(title);
    let file_path = format!("notes/{}.md", slug);

    let content = format!(
        r#"---
title: {}
date: {}
tags: []
---

# {}

<!-- Start writing your note below -->

"#,
        title, date, title
    );

    if !Path::new("notes").exists() {
        fs::create_dir("notes").expect("Failed to create directory");
    }

    fs::write(&file_path, content)?;
    println!("note created: {}", file_path);

    Ok(())
}
