use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "zet")]
#[command(about = "A simple note-taking CLI tool in Rust", long_about = None)]

pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    New {
        /// The title of the note
        title: String,
    },
    /// List all notes
    List,
    /// Edit an existing note in your $EDITOR
    Edit {
        /// The slug of the note (without .md)
        slug: String,
    },
    /// View the selected notes as Markdown
    View {
        /// the slug of the note (without .md)
        slug: String,
    },
    /// Launch interactive fuzzy search UI
    Search,
}
