pub fn slugify(title: &str) -> String {
    title.to_lowercase().replace(' ', "-")
}

pub fn validate_tags(raw: &str) -> Result<Vec<String>, String> {
    let mut seen = std::collections::HashSet::new();
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

        if !seen.insert(tag.to_lowercase()) {
            return Err(format!("Duplicate tag detected: `{}`", tag));
        }

        clean_tags.push(tag.to_string());
    }

    Ok(clean_tags)
}
