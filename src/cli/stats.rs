use crate::core::usecases::list as list_usecase;
use crate::errors::AppError;
use crate::storage::local_repo::LocalMarkdownRepo;

pub fn get_stats() -> Result<(), AppError> {
    let repo = LocalMarkdownRepo::default();
    let notes = list_usecase::list_notes(&repo)?;
    let note_count = notes.len();
    println!("Total notes: {}", note_count);

    Ok(())
}
