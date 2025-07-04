use crate::utils::frontmatter::{Frontmatter, detect_duplicate_tags};
use std::error::Error;

pub fn slugify(title: &str) -> String {
    title.to_lowercase().replace(' ', "-")
}

pub fn validate_tags(raw: &str) -> Result<Vec<String>, String> {
    let mut clean_tags = Vec::new();

    for tag in raw.split(',') {
        let tag = tag.trim();

        if tag.is_empty() {
            continue;
        }

        if !is_valid_tag(tag) {
            return Err(format!(
                "Tag `{}` contains invalid characters. Only alphanumeric, `_`, `-` are allowed.",
                tag
            ));
        }

        clean_tags.push(tag.to_string());
    }

    detect_duplicate_tags(&clean_tags);

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

fn is_valid_tag(tag: &str) -> bool {
    tag.chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}
