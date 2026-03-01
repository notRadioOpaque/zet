mod args;
mod cli;
mod core;
mod errors;
mod storage;
mod tui;

use clap::Parser;

use args::{Cli, Commands};

use cli::edit::edit_note;
use cli::lint::lint_notes;
use cli::list::list_notes;
use cli::new::create_note;
use cli::tui::run_tui;
use cli::view::view_note;

fn main() {
    let cli: Cli = Cli::parse();

    let result = match &cli.command {
        Commands::New { title, tags } => {
            let transformed_title = title.trim_matches(|c| c == '"' || c == '\'');
            create_note(transformed_title, tags.as_deref())
        }
        Commands::List => list_notes(),
        Commands::Edit { slug } => edit_note(slug),
        Commands::View { slug } => view_note(slug),
        Commands::Lint { fix } => lint_notes(*fix),
        Commands::Tui => {
            if let Err(err) = run_tui() {
                eprintln!("TUI error: {}", err);
                std::process::exit(1);
            }

            Ok(())
        }
    };

    if let Err(err) = result {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
