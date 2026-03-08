use termimad::MadSkin;

use crate::cli::validate_slug;
use crate::core::usecases::read as read_usecase;
use crate::errors::AppError;
use crate::storage::local_repo::LocalMarkdownRepo;

pub fn view_note(slug: &str) -> Result<(), AppError> {
    let slug = validate_slug(slug)?;

    let repo = LocalMarkdownRepo::default();
    let note = read_usecase::read_note(&repo, slug)?;

    let skin = MadSkin::default();
    println!("Viewing: notes/{}.md\n", slug);
    skin.print_text(&note.body);

    Ok(())
}
