use termimad::MadSkin;

use crate::core::usecases::read as read_usecase;
use crate::errors::AppError;
use crate::storage::local_repo::LocalMarkdownRepo;

pub fn view_note(slug: &str) -> Result<(), AppError> {
    let slug = slug.trim();

    if slug.is_empty() {
        return Err(AppError::InvalidInput("Slug cannot be empty".to_string()));
    }

    if !slug.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
        return Err(AppError::InvalidInput(
            "Please use a valid string as slug".to_string(),
        ));
    }

    let repo = LocalMarkdownRepo::default();
    let note = read_usecase::read_note(&repo, slug)?;

    let skin = MadSkin::default();
    println!("Viewing: notes/{}.md\n", slug);
    skin.print_text(&note.body);

    Ok(())
}
