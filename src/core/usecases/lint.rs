use chrono::NaiveDate;

use crate::core::errors::CoreError;
use crate::core::repository::NoteRepository;
use crate::core::validators::{dedup_tags, has_duplicate_tags};
use crate::storage::frontmatter::{build_note_content, parse_frontmatter_and_body};

#[derive(Debug, Clone)]
pub struct LintIssue {
    pub slug: String,
    pub message: String,
}

pub fn lint_notes<R: NoteRepository>(repo: &R, fix: bool) -> Result<Vec<LintIssue>, CoreError> {
    let mut issues = Vec::new();

    for slug in repo.list_note_slugs()? {
        let raw = repo.read_raw_note(&slug)?;

        let (mut frontmatter, body) = match parse_frontmatter_and_body(&raw, &slug) {
            Ok(parsed) => parsed,
            Err(err) => {
                issues.push(LintIssue {
                    slug: slug.clone(),
                    message: err,
                });
                continue;
            }
        };

        if frontmatter.title.trim().is_empty() {
            issues.push(LintIssue {
                slug: slug.clone(),
                message: "Missing required frontmatter field `title`".to_string(),
            });
        }

        if frontmatter.date.trim().is_empty() {
            issues.push(LintIssue {
                slug: slug.clone(),
                message: "Missing required frontmatter field `date`".to_string(),
            });
        } else if NaiveDate::parse_from_str(&frontmatter.date, "%Y-%m-%d").is_err() {
            issues.push(LintIssue {
                slug: slug.clone(),
                message: format!(
                    "Invalid `date` format `{}`. Expected YYYY-MM-DD",
                    frontmatter.date
                ),
            });
        }

        if has_duplicate_tags(&frontmatter.tags) {
            issues.push(LintIssue {
                slug: slug.clone(),
                message: "Duplicate tags found".to_string(),
            });

            if fix {
                dedup_tags(&mut frontmatter.tags);
                let new_content = build_note_content(&frontmatter, &body)?;
                repo.write_raw_note(&slug, &new_content)?;
            }
        }
    }

    Ok(issues)
}
