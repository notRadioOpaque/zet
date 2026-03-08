use crate::errors::AppError;

pub mod edit;
pub mod lint;
pub mod list;
pub mod new;
pub mod search;
pub mod stats;
pub mod tui;
pub mod view;

pub(crate) fn validate_slug(slug: &str) -> Result<&str, AppError> {
    let slug = slug.trim();

    if slug.is_empty() {
        return Err(AppError::InvalidInput("Slug cannot be empty".to_string()));
    }

    if !slug.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
        return Err(AppError::InvalidInput(
            "Please use a valid string as slug".to_string(),
        ));
    }

    Ok(slug)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_slug_rejects_empty() {
        assert!(validate_slug("   ").is_err());
    }

    #[test]
    fn validate_slug_rejects_invalid_chars() {
        assert!(validate_slug("a/b").is_err());
        assert!(validate_slug("hello!").is_err());
    }

    #[test]
    fn validate_slug_accepts_valid_slug_and_trims() {
        let slug = validate_slug("  hello-world  ").unwrap();
        assert_eq!(slug, "hello-world");
    }
}
