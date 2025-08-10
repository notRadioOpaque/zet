mod cli;
mod commands;
mod tui;
mod utils;

use clap::Parser;
use cli::{Cli, Commands};

use crate::commands::lint::lint_notes;
use crate::commands::tui::run_tui;
use commands::edit::edit_note;
use commands::list::list_notes;
use commands::new::create_note;
// use commands::search::interactive_search;
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
        // Commands::Search => interactive_search(),
        Commands::Lint { fix } => {
            if let Err(err) = lint_notes(*fix) {
                eprintln!("Error: {}", err);
                std::process::exit(1);
            }

            Ok(())
        }
        Commands::Tui => {
            if let Err(err) = run_tui() {
                eprintln!("TUI error: {}", err);
                std::process::exit(1);
            }

            Ok(())
        }
    };

    if let Err(err) = result {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
