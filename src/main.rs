mod cli;
mod commands;
mod utils;

use clap::Parser;
use cli::{Cli, Commands};

use commands::edit::edit_note;
use commands::list::list_notes;
use commands::new::create_note;
use commands::search::interactive_search;
use commands::view::view_note;

fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::New { title, tags } => {
            let transformed_title = title.trim_matches(|c| c == '"' || c == '\'');
            create_note(transformed_title, tags.as_deref())
        }
        Commands::List => list_notes(),
        Commands::Edit { slug } => edit_note(slug),
        Commands::View { slug } => view_note(slug),
        Commands::Search => interactive_search(),
    };

    if let Err(err) = result {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
