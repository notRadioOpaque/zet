use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Struct to represent parsed YAML metadata
#[derive(Debug, Deserialize, Serialize)]
pub struct Frontmatter {
    pub title: String,
    pub date: String,
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Parse YAML Frontmatter block from note content
/// Returns a typed Frontmatter struct or a user-friendly error
pub fn parse_frontmatter(content: &str, slug: &str) -> Result<Frontmatter, String> {
    let sections: Vec<&str> = content.splitn(3, "---").collect();

    if sections.len() < 3 {
        return Err(format!("No frontmatter found in note `{}`", slug));
    }

    let yaml_block = sections[1];

    let frontmatter: Frontmatter = serde_yaml::from_str(yaml_block)
        .map_err(|e| format!("Failed to parse YAML in `{}`: {}", slug, e))?;

    Ok(frontmatter)
}

/// Checks for case-insensitive duplicate tags
/// Returns the first duplicate if found
pub fn detect_duplicate_tags(tags: &[String]) -> Option<String> {
    let mut seen = HashSet::new();

    for tag in tags {
        let normalized = tag.to_lowercase();

        if !seen.insert(normalized) {
            return Some(tag.clone());
        }
    }

    None
}

/// Deduplicates tags (case-insensitive) preserving order
pub fn dedup_tags(tags: &mut Vec<String>) {
    let mut seen = HashSet::new();

    tags.retain(|tag| seen.insert(tag.to_lowercase()));
}
