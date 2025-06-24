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

    match &cli.command {
        Commands::New { title } => {
            let transformed_title = title.trim_matches(|c| c == '"' || c == '\'');

            create_note(transformed_title);
        }
    }
}

fn slugify(title: &str) -> String {
    title.to_lowercase().replace(' ', "-")
}

fn create_note(title: &str) {
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

    fs::write(&file_path, content).expect("Failed to write note");

    println!("note created: {}", file_path);
}
