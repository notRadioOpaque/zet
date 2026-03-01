use serde::{Deserialize, Serialize};

use crate::core::errors::CoreError;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Frontmatter {
    pub title: String,
    pub date: String,
    #[serde(default)]
    pub tags: Vec<String>,
}

pub fn parse_frontmatter_and_body(
    content: &str,
    slug: &str,
) -> Result<(Frontmatter, String), String> {
    let sections: Vec<&str> = content.splitn(3, "---").collect();

    if sections.len() < 3 {
        return Err(format!("No frontmatter found in note `{}`", slug));
    }

    let yaml_block = sections[1];
    let frontmatter: Frontmatter = serde_yaml::from_str(yaml_block)
        .map_err(|e| format!("Failed to parse YAML in `{}`: {}", slug, e))?;

    Ok((frontmatter, sections[2].to_string()))
}

pub fn build_note_content(frontmatter: &Frontmatter, body: &str) -> Result<String, CoreError> {
    let yaml = serde_yaml::to_string(frontmatter)?;
    Ok(format!(
        "---\n{}---\n\n{}",
        yaml,
        body.trim_start_matches('\n')
    ))
}
