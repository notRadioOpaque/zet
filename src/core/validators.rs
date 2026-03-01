use std::collections::HashSet;

pub fn slugify(title: &str) -> String {
    title
        .trim()
        .to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

pub fn validate_tags(raw: &str) -> Result<Vec<String>, String> {
    let mut clean_tags = Vec::new();

    for tag in raw.split(',') {
        let tag = tag.trim();

        if tag.is_empty() {
            continue;
        }

        if !tag
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        {
            return Err(format!(
                "Tag `{}` contains invalid characters. Only alphanumeric, `_`, `-` are allowed.",
                tag
            ));
        }

        clean_tags.push(tag.to_string());
    }

    Ok(clean_tags)
}

pub fn dedup_tags(tags: &mut Vec<String>) {
    let mut seen = HashSet::new();
    tags.retain(|tag| seen.insert(tag.to_lowercase()));
}

pub fn has_duplicate_tags(tags: &[String]) -> bool {
    let mut seen = HashSet::new();
    for tag in tags {
        if !seen.insert(tag.to_lowercase()) {
            return true;
        }
    }
    false
}
