use crate::core::usecases::list as list_usecase;
use crate::errors::AppError;
use crate::storage::local_repo::LocalMarkdownRepo;

pub fn list_notes() -> Result<(), AppError> {
    let repo = LocalMarkdownRepo::default();
    let notes = list_usecase::list_notes(&repo)?;

    if notes.is_empty() {
        println!("No notes found. Try creating one first with `zet new <title>`.");
        return Ok(());
    }

    println!("Notes (slug | date | tags):\n");
    for (i, note) in notes.iter().enumerate() {
        let tags_display = if note.tags.is_empty() {
            String::new()
        } else {
            format!(" | tags: {}", note.tags.join(", "))
        };

        println!("{}. {} | {}{}", i + 1, note.slug, note.date, tags_display);
    }

    Ok(())
}
