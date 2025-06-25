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
        /// the title of the note
        title: String,
    },
    // other commands to be added here...
}
