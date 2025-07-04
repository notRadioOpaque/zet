use crate::utils::frontmatter::Frontmatter;
use std::error::Error;

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

pub fn build_note_content(frontmatter: &Frontmatter, body: &str) -> Result<String, Box<dyn Error>> {
    let tags_inline = format!("[{}]", frontmatter.tags.join(", "));

    let yaml = format!(
        "title: {}\ndate: {}\ntags: {}\n",
        frontmatter.title, frontmatter.date, tags_inline
    );

    let content = format!("---\n{}---\n\n{}", yaml, body);
    Ok(content)
}
