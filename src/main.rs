use chrono::Local;
use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(name = "zet")]
#[command(about = "A simple note-taking CLI tool in Rust", long_about = None)]

struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    New {
        /// the title of the note
        title: String,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::New { title } => {
            let transformed_title = title.trim_matches(|c| c == '"' || c == '\'');
            create_note(transformed_title)
        }
    };

    if let Err(err) = result {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

fn slugify(title: &str) -> String {
    title.to_lowercase().replace(' ', "-")
}

fn create_note(title: &str) -> Result<(), Box<dyn std::error::Error>> {
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
