use crate::core::usecases::lint as lint_usecase;
use crate::errors::AppError;
use crate::storage::local_repo::LocalMarkdownRepo;

pub fn lint_notes(fix: bool) -> Result<(), AppError> {
    let repo = LocalMarkdownRepo::default();
    let issues = lint_usecase::lint_notes(&repo, fix)?;

    if issues.is_empty() {
        println!("No lint issues found.");
        return Ok(());
    }

    for issue in &issues {
        println!("{}: {}", issue.slug, issue.message);
    }

    if fix {
        println!("Applied available auto-fixes.");
    }

    Ok(())
}
