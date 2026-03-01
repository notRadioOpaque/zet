#[derive(Debug, Clone)]
pub struct Note {
    pub slug: String,
    pub title: String,
    pub date: String,
    pub tags: Vec<String>,
    pub body: String,
}
