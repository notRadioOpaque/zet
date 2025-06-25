pub fn slugify(title: &str) -> String {
    title.to_lowercase().replace(' ', "-")
}
