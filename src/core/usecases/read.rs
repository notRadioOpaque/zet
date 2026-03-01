use crate::core::errors::CoreError;
use crate::core::note::Note;
use crate::core::repository::NoteRepository;

pub fn read_note<R: NoteRepository>(repo: &R, slug: &str) -> Result<Note, CoreError> {
    repo.read_note(slug)
}
