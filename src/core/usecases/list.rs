use crate::core::errors::CoreError;
use crate::core::note::Note;
use crate::core::repository::NoteRepository;

pub fn list_notes<R: NoteRepository>(repo: &R) -> Result<Vec<Note>, CoreError> {
    repo.list_notes()
}
