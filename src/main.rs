mod cli;
mod commands;
mod utils;

use clap::Parser;
use cli::{Cli, Commands};

use commands::edit::edit_note;
use commands::list::list_notes;
use commands::new::create_note;

fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::New { title } => {
            let transformed_title = title.trim_matches(|c| c == '"' || c == '\'');
            create_note(transformed_title)
        }
        Commands::List => list_notes(),
        Commands::Edit { slug } => edit_note(slug),
    };

    if let Err(err) = result {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
