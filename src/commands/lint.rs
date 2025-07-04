use crate::utils::frontmatter::{dedup_tags, parse_frontmatter};
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn lint_notes(fix: bool) -> Result<(), Box<dyn std::error::Error>> {
    let notes_dir = Path::new("notes");
    let entries = fs::read_dir(notes_dir)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) != Some("md") {
            continue;
        }

        let slug = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");

        let content = fs::read_to_string(&path)?;

        let mut frontmatter = match parse_frontmatter(&content, slug) {
            Ok(fm) => fm,
            Err(err) => {
                eprintln!("⚠ {}: {}", slug, err);
                continue;
            }
        };

        let original_tags = frontmatter.tags.clone();
        dedup_tags(&mut frontmatter.tags);

        if original_tags != frontmatter.tags {
            if fix {
                // Split body
                let sections: Vec<&str> = content.splitn(3, "---").collect();
                let body = sections.get(2).unwrap_or(&"");

                let yaml = serde_yaml::to_string(&frontmatter)?;
                let new_content = format!("---\n{}---{}", yaml, body);

                let mut file = fs::File::create(&path)?;
                file.write_all(new_content.as_bytes())?;

                println!("✅ Fixed duplicate tags in `{}`", slug);
            } else {
                println!("⚠ Duplicate tags in `{}`", slug);
            }
        }
    }

    Ok(())
}
